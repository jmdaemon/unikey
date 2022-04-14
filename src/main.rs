use log::{debug, error, info, warn};
use clap::{Arg, Command};
use toml::Value;
use failure::Error;
use std::fs::read_to_string;
use utility::files::{write_file};
use utility::layout::{Keys, KeyMap, Layout, create_layout, create_evdev, create_lst};
use std::process::exit;

pub fn build_cli() -> clap::Command<'static> {
    // Create Unikey CLI
    let app = Command::new("Unikey")
        .version("0.1.1")
        .author("Joseph Diza <josephm.diza@gmail.com>")
        .about("Create linux xkb keyboard layouts")
        .arg(Arg::new("kb_layout_fp")
            .required(true)
            .help("File path to the keyboard layout toml file"))
        .arg(Arg::new("output")
            .default_value("./layouts")
            .help("Output directory"));
    app
}

const ROWS: [&str; 4] = ["e", "d", "c", "b"];

/// Returns a vector of keys from a keyboard layout
pub fn parse_rows(keyboard_layout: Value) -> Vec<Keys> {
    // Create a vector of keys
    let mut row_keys: Vec<Keys> = vec![];

    // For every row in the row of keys
    for row_name in ROWS {
        // Get the current row
        let row = &keyboard_layout["rows"][&row_name];

        // Get all the keys values for that row
        let key_values = row.as_array().unwrap();

        // Convert the key values to a vector of strings
        let mut keys: Vec<String> = vec![];
        for key in key_values {
            keys.push(key.to_string());
        }
        // Add the keys for that row to the vector
        row_keys.push(Keys { keys: keys });
    }
    row_keys
}

pub fn parse_misc() {
}

/// Display keyboard config debug info
pub fn show_kb_layout(
    kb_layout_fp: &str, kb_name: &str, kb_desc: &str, kb_layout_contents: &str) {

    println!("Parsing keyboard config");
    debug!("Keyboard Config\n");
    debug!("{}", "=".repeat(16));
    debug!("Keyboard Layout File: {}\n", kb_layout_fp);
    debug!("Keyboard Name       : {}\n", kb_name);
    debug!("Keyboard Descrption : {}\n", kb_desc);
    debug!("{}", "=".repeat(16));

    debug!("Keyboard Config Contents: ");
    debug!("{}", "=".repeat(16));
    debug!("{}\n", kb_layout_contents);
    debug!("{}", "=".repeat(16));
}

fn main() -> Result<(), Error> {
    // Use logging
    pretty_env_logger::init();

    // Create command line interface
    let app = build_cli();
    let matches = app.get_matches();

    // Parse arguments
    let kb_output_fp = matches.value_of("output").unwrap();
    let kb_layout_fp = matches
        .value_of("kb_layout_fp")
        .expect("Keyboard layout config was not found.");

    let kb_layout_contents = &(read_to_string(kb_layout_fp)
        .expect("Could not read keyboard layout config"));

    let kb_layout: Value = toml::from_str(&kb_layout_contents)
        .expect("Could not parse keyboard layout config");

    let kb_config = &kb_layout["config"];

    // Parse keyboard config
    // Default to us, English (US) layout
    let kb_name = kb_config["name"].as_str().unwrap_or("us");
    let kb_desc = kb_config["desc"].as_str().unwrap_or("English (US)");

    show_kb_layout(kb_output_fp, kb_name, kb_desc, kb_layout_contents);

    // Initializes Tera templates
    let layout: Layout = Layout::new(kb_name, kb_desc);
    let ekeys = &kb_layout["rows"]["e"].as_array();
    for key in ekeys.unwrap().iter() {
        //let skey = key.to_string();
        //println!("{}", skey)
        println!("Key: {}", key)
    }

    let kmap: KeyMap = KeyMap::new(kb_layout);
    //println!("{:?}", kmap.keys.keys);
    for rows in kmap.keys.iter() {
        for keys in rows.keys.iter() {
            println!("{}", keys.to_string());
        }
    }

    exit(0);
    let rendered_layout = create_layout(&kb_layout, &layout, false);

    // Render our variables into the templates
    let evdev = create_evdev(&layout);
    let base_lst = create_lst(&layout, "base.lst");
    let evdev_lst = create_lst(&layout, "evdev.lst");

    // Write Linux XKB templates to layouts output directory
    write_file(&rendered_layout, kb_name);
    write_file(&evdev, "evdev.xml");
    write_file(&base_lst, "base.lst");
    write_file(&evdev_lst, "evdev.lst");
    
    Ok(())
}

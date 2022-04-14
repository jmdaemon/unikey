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
        .arg(Arg::new("v")
            .short('v')
            .long("verbose")
            .required(false)
            .help("Show verbose output"))
        .arg(Arg::new("keyboard_layout")
            .help("Specify the file path to the keyboard layout config"));
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

fn main() -> Result<(), Error> {
    // Use logging
    pretty_env_logger::init();
    let app = build_cli();

    // Print help message if user inputs -vv
    let mut borrow_app = app.clone();
    let matches = app.get_matches();
    let verbose;
    match matches.occurrences_of("v") {
        0 => verbose = false,
        1 => verbose = true,
        _ => {
            borrow_app.print_help()?;
            println!("");
            std::process::exit(1);
        }
    }

    // Parse arguments
    let kb_layout_fp = matches
        .value_of("keyboard_layout")
        .expect("Keyboard layout config was not found.");

    let kb_layout_contents = read_to_string(kb_layout_fp)
        .expect("Could not read keyboard layout config");

    let kb_layout: Value = toml::from_str(&kb_layout_contents)
        .expect("Could not parse keyboard layout config");

    let kb_config = &kb_layout["config"];

    // Parse keyboard config
    // Default to us, English (US) layout
    let kb_name = kb_config["name"].as_str().unwrap_or("us");
    let kb_desc = kb_config["desc"].as_str().unwrap_or("English (US)");

    // Display keyboard config info
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

    //if verbose {
        //display("=== Contents ===", format!("{}", kb_layout_contents));
        //display("=== Keyboard Layout ===", format!("{:?}", &kb_layout));
        //display("=== Rows ===", format!("{:?}", &kb_layout["rows"]));
        //display("=== Row E ===", format!("{:?}", &kb_layout["rows"]["e"]));
    //}

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
    let rendered_layout = create_layout(&kb_layout, &layout, verbose);

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

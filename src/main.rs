extern crate clap;

use clap::{Arg, Command};
use toml::Value;
use failure::Error;
use utils::files::{read_file, write_file};
use utils::layout::{KeyMap, Layout, create_layout, create_evdev, create_lst};
use std::process::exit;

pub fn boxtitle(title: &str) -> (String, String) {
    (title.to_string(), "=".repeat(title.len()))
}

pub fn display(title: &str, msg: String) {
    let boxtitle = boxtitle(title);
    println!("{}", boxtitle.0);
    println!("{}", msg);
    println!("{}\n", boxtitle.1);
}

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
            .help("Show verbose output")
            )
        .arg(Arg::new("keyboard_layout")
            .help("Specify the file path to the keyboard layout config"))
        .arg(Arg::new("name")
            .help("Specify the name of your keyboard layout"))
        .arg(Arg::new("desc")
            .help("Give a brief description for keyboard layout name. Ex. English (US)"));
    app
}

fn main() -> Result<(), Error> {
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
    let filename = matches.value_of("keyboard_layout").expect("Keyboard layout file was not found.");
    let contents = read_file(filename);
    let keyboard_layout: Value = toml::from_str(&contents)?;
    let config = &keyboard_layout["config"];

    // Parse keyboard config
    let name = config.get("name").unwrap().as_str().unwrap_or("us");
    let desc = config.get("desc").unwrap().as_str().unwrap_or("English (US)");
    let keyboard_name = matches.value_of("name").unwrap_or(name);
    let keyboard_desc = matches.value_of("desc").unwrap_or(desc);

    // Display keyboard config info
    display("================================",
        format!(
            concat!(
            "Keyboard Layout File  : {}\n",
            "Keyboard Name         : {}\n",
            "Keyboard Description  : {}"), filename, keyboard_name, keyboard_desc
            ));

    if verbose {
        display("=== Contents ===", format!("{}", contents));
        display("=== Keyboard Layout ===", format!("{:?}", &keyboard_layout));
        display("=== Rows ===", format!("{:?}", &keyboard_layout["rows"]));
        display("=== Row E ===", format!("{:?}", &keyboard_layout["rows"]["e"]));
    }

    // Initializes Tera templates
    let layout: Layout = Layout::new(keyboard_name, keyboard_desc);
    let ekeys = &keyboard_layout["rows"]["e"].as_array();
    for key in ekeys.unwrap().iter() {
        //let skey = key.to_string();
        //println!("{}", skey)
        println!("Key: {}", key)
    }

    let kmap: KeyMap = KeyMap::new(keyboard_layout);
    //println!("{:?}", kmap.keys.keys);
    for rows in kmap.keys.iter() {
        for keys in rows.keys.iter() {
            println!("{}", keys.to_string());
        }
    }
    exit(0);
    let rendered_layout = create_layout(&keyboard_layout, &layout, verbose);

    // Render our variables into the templates
    let evdev = create_evdev(&layout);
    let base_lst = create_lst(&layout, "base.lst");
    let evdev_lst = create_lst(&layout, "evdev.lst");

    // Write Linux XKB templates to layouts output directory
    write_file(&rendered_layout, keyboard_name);
    write_file(&evdev, "evdev.xml");
    write_file(&base_lst, "base.lst");
    write_file(&evdev_lst, "evdev.lst");
    
    Ok(())
}

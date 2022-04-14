// Unikey Modules
use unikey::keyboard::{Keys, KeyboardLayout, show_kb_layout};
use unikey::tmpl::{init_tera};
use unikey::parse::{parse_rows, parse_misc};
use unikey::linux::{populate_linux_kb};

// Third Party Crates
use log::error;
use toml::Value;
use clap::{Arg, Command};

// Standard Library
use std::process::exit;
use std::collections::HashMap;
use std::fs::{read_to_string, write, create_dir_all};

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
            .help("Output directory"))
        .arg(Arg::new("dryrun")
            .required(false)
            .short('d')
            .long("dryrun")
            .help("Don't output files to disk"))
        .arg(Arg::new("kb_type")
            .default_value("linux")
            .short('t')
            .long("type")
            .help("Type of keyboard layout. Types: [linux, apple, windows]."));
    app
}

fn main() {
    // Use logging
    pretty_env_logger::init();

    // Create command line interface
    let app = build_cli();
    let matches = app.get_matches();

    // Parse arguments
    let dryrun = matches.is_present("dryrun");
    if dryrun {
        println!("Showing dryrun of output keyboard layout");
    }

    let kb_output_fp = matches.value_of("output").unwrap();
    let kb_type = matches.value_of("kb_type").unwrap();
    let kb_layout_fp = matches
        .value_of("kb_layout_fp")
        .expect("Keyboard layout config was not found.");

    let kb_layout_contents = &(read_to_string(kb_layout_fp)
        .expect("Could not read keyboard layout config"));

    println!("Parsing keyboard config");
    let kb_layout: Value = toml::from_str(&kb_layout_contents)
        .expect("Could not parse keyboard layout config");

    let kb_config = &kb_layout["config"];

    // Parse keyboard config
    // Default to us, English (US) layout
    let kb_name = kb_config["name"].as_str().unwrap_or("us");
    let kb_desc = kb_config["desc"].as_str().unwrap_or("English (US)");

    show_kb_layout(kb_name, kb_desc, kb_layout_contents);

    // Parse keys in keyboard config
    // Note that misc keys are handled separately from the normal keys
    let kb_rows: HashMap<&str, Keys> = parse_rows(&kb_layout);
    let kb_misc: HashMap<&str, String> = parse_misc(&kb_layout);

    // Store output in KeyboardLayout
    let kb = KeyboardLayout::new(kb_name, kb_desc, kb_rows, kb_misc);

    // Initialize Tera
    let tera = init_tera(kb_type);

    let mut rendered: HashMap<String, String> = HashMap::new();
    match kb_type {
        "linux" => {
            // Populate templates with values from keyboard config
            rendered = populate_linux_kb(&kb, dryrun, &tera);
        }
        _ => {
            // The input can't be/should not be null here
            error!("kb_type was not set to a default value");
        }
    }

    if dryrun {
        // Exit early after printing the template
        exit(0); 
    }

    // Create directory if it doesn't exist
    println!("Writing rendered templates to {}", kb_output_fp);
    create_dir_all(kb_output_fp).expect(&format!("Directory {} could not be created", kb_output_fp));

    // Write the template to the output folder
    for (filename, contents) in rendered {
        let fp = format!("{}/{}", kb_output_fp, filename);
        write(fp, contents).expect("Unable to write file");
    }
}

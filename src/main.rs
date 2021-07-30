use clap::{Arg, App, AppSettings};
use toml::{Value};
use tera::{Tera, Context};
use failure::Error;

use std::collections::HashMap;

use utils::files::{read_file, write_file};

fn create_layout(keyboard_layout: Value, layout_name: String, layout_desc: String, verbose: bool) -> String {
    let rows = ["e", "d", "c", "b", "misc"];
    let mut rows_table = HashMap::new();

    for row in rows { rows_table.insert(row, keyboard_layout["rows"][&row].as_table()); };
    let tera = match Tera::new("templates/**/*.tmpl") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let mut context = Context::new();
    context.insert("layout_name", &layout_name);
    context.insert("layout_desc", &layout_desc);
    if verbose {
        println!("Using layout name: {}", &layout_name);
        for (row, keys) in rows_table {
            println!("\nRow: {}", row);
            for (key, value) in keys.unwrap() {
                let key = str::replace(key, "-", "_");
                println!("Key: {}, Value: {}", key, value);
                context.insert([row, key.as_str()].join("_"), &value);
            }
        }
    }
    else {
        for (row, keys) in rows_table {
            for (key, value) in keys.unwrap() {
                let key = str::replace(key, "-", "_");
                context.insert([row, key.as_str()].join("_"), &value);
            }
        }
    }

    println!("\n=== Linux Keyboard Layout ===\n");
    let rendered = tera.render("layout.tmpl", &context).expect("Template failed to render");
    println!("\n{}", rendered);
    return rendered;
}

fn main() -> Result<(), Error> {
    let app =
        App::new("Unikey")
        .setting(AppSettings::ArgRequiredElseHelp)
        .help_message("Show this message")
        .version_message("Show the current unikey version")
        .version("0.1.0")
        .author("Joseph Diza <josephm.diza@gmail.com>")
        .about("Create linux xkb keyboard layouts")
        .arg(Arg::with_name("v")
            .short("v")
            .long("verbose")
            .multiple(true)
            .help("Show verbose output"))
        .arg(Arg::with_name("keyboard_layout")
            .help("Specify the file path to the keyboard layout config")
            .required(true)
            .index(1))
        .arg(Arg::with_name("name")
            .help("Specify the name of your keyboard layout")
            .required(true)
            .index(2))
        .arg(Arg::with_name("desc")
            .help("Give a brief description for keyboard layout name. Ex. English (US)")
            .required(true)
            .index(3));

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

    let filename = matches.value_of("keyboard_layout").unwrap();
    let desc = matches.value_of("desc").unwrap();
    println!("Using keyboard layout file: {} with description: {}", filename, desc);
    let contents = read_file(filename);
    let keyboard_layout: Value = toml::from_str(&contents)?;

    if verbose {
        println!("=== Contents ===\n{}", contents);
        println!("=== Keyboard Layout === \n{:?}\n", &keyboard_layout);
        println!("=== Rows === \n{:?}\n", &keyboard_layout["rows"]);
        println!("=== Row E === \n{:?}\n", &keyboard_layout["rows"]["e"]);
    }

    // Specify keyboard names and description as args or in key.layout.toml file
    //let keyboard_name = matches.value_of("keyboard_layout").unwrap()
        //.split(".").next().expect("Keyboard file name layout is improperly formatted");
    //let rendered_layout = create_layout(keyboard_layout, keyboard_name.to_string(), desc.to_string(), verbose);
    let keyboard_name = matches.value_of("name").unwrap();
    let rendered_layout = create_layout(keyboard_layout, keyboard_name.to_string(), desc.to_string(), verbose);
    write_file(rendered_layout, "math");
    
    Ok(())
}

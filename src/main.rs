use clap::{Arg, App, AppSettings};
use toml::{Value, value::Table};
use tera::{Tera, Context};
use failure::Error;

use std::collections::HashMap;
use std::process::exit;

use utils::files::{read_file, write_file};

fn init_context(rows_table: HashMap<&str, Option<&Table>>, layout_name: String, layout_desc: String, verbose: bool) -> tera::Context {
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
    println!("");
    return context;
}

fn create_layout(keyboard_layout: Value, layout_name: String, layout_desc: String, verbose: bool) -> String {
    let rows = ["e", "d", "c", "b", "misc"];
    let mut rows_table = HashMap::new();

    for row in rows { rows_table.insert(row, keyboard_layout["rows"][&row].as_table()); };
    let tera = match Tera::new("templates/**/*.tmpl") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Parsing error(s): {}", e);
            exit(1);
        }
    };

    let context = init_context(rows_table, layout_name, layout_desc, verbose);
    println!("=== Linux Keyboard Layout ===");
    let rendered = tera.render("layout.tmpl", &context).expect("Template failed to render");
    println!("{}", rendered);
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
            .takes_value(true)
            .index(2))
        .arg(Arg::with_name("desc")
            .help("Give a brief description for keyboard layout name. Ex. English (US)")
            .takes_value(true)
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

    let filename = matches.value_of("keyboard_layout").expect("Keyboard layout file was not found.");
    let contents = read_file(filename);
    let keyboard_layout: Value = toml::from_str(&contents)?;
    let config = keyboard_layout["config"].clone();

    let name = config.get("name").unwrap().as_str().unwrap_or("us");
    let desc = config.get("desc").unwrap().as_str().unwrap_or("English (US)");
    let keyboard_name = matches.value_of("name").unwrap_or(name);
    let keyboard_desc = matches.value_of("desc").unwrap_or(desc);

    println!("================================");
    println!("Keyboard Layout File  : {}", filename);
    println!("Keyboard Name         : {}", keyboard_name);
    println!("Keyboard Description  : {}", keyboard_desc);

    if verbose {
        println!("=== Contents ===\n{}", contents);
        println!("=== Keyboard Layout === \n{:?}\n", &keyboard_layout);
        println!("=== Rows === \n{:?}\n", &keyboard_layout["rows"]);
        println!("=== Row E === \n{:?}\n", &keyboard_layout["rows"]["e"]);
    }

    let rendered_layout = create_layout(keyboard_layout, keyboard_name.to_string(), keyboard_desc.to_string(), verbose);
    write_file(rendered_layout, "math");
    
    Ok(())
}

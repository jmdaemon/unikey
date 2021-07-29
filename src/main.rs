use clap::{Arg, App};
use toml::{Value};
use tera::{Tera, Context};
use failure::Error;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;

fn read_file(filename: &str) -> String {
    return fs::read_to_string(filename)
        .expect("Unable to read keyboard layout file").to_owned();
}

fn write_file(keyboard_layout: String, filename: &str) {
    let mut f = File::create(["layouts", filename].join("/")).expect("Unable to create file");
    f.write_all(keyboard_layout.as_bytes()).expect("Unable to write data");
}

fn create_layout(keyboard_layout: Value, layout_name: String) -> String {
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
    println!("Using layout name: {}", &layout_name);

    for (row, keys) in rows_table {
        for (key, value) in keys.unwrap() {
            let key = str::replace(key, "-", "_");
            println!("Key: {}, Value: {}", key, value);
            context.insert([row, key.as_str()].join("_"), &value);
        }
    }

    println!("\n=== Linux Keyboard Layout ===\n");
    let rendered = tera.render("layout.tmpl", &context).expect("Template failed to render");
    println!("\n{}", rendered);
    return rendered;
}

fn main() -> Result<(), Error> {
    let matches =
        App::new("Unikey") 
        .version("0.1.0")
        .author("Joseph Diza <josephm.diza@gmail.com>")
        .about("Create linux xkb keyboard layouts")
        .arg(Arg::with_name("v")
            .short("v")
            .long("verbose")
            .multiple(true)
            .help("Show verbose output"))
        .arg(Arg::with_name("filename")
            .help("Specify [keyboard_name].layout.toml file read from")
            .required(true)
            .index(1))
        .get_matches();

    match matches.occurrences_of("v") {
        0 => println!("Don't Show verbose output"),
        1 => println!("Do Show verbose output"),
        _ => println!("Show help message"), }

    let filename = matches.value_of("filename").unwrap();
    println!("Using keyboard layout file: {}", filename);

    let contents = read_file(filename);
    println!("With text:\n{}", contents);

    let keyboard_layout: Value = toml::from_str(&contents)?;
    println!("=== Keyboard Layout === \n{:?}\n", &keyboard_layout);
    println!("=== Rows === \n{:?}\n", &keyboard_layout["rows"]);
    println!("=== Row E === \n{:?}\n", &keyboard_layout["rows"]["e"]);

    let key_1 = &keyboard_layout["rows"]["e"]["key-1"];
    println!("Key-1: {:?}\n", key_1);
    assert_eq!(key_1.as_str(), Some("1"));

    let rendered_layout = create_layout(keyboard_layout, "math".to_string());
    write_file(rendered_layout, "math");
    
    Ok(())
}

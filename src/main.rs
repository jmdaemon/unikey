use clap::{Arg, App};
use toml::{Value, de::Error};

use std::fs;

fn read_file(filename: &str) -> String {
    return fs::read_to_string(filename)
        .expect("Unable to read keyboard layout file").to_owned();
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

    let e_keys = &keyboard_layout["rows"]["e"].as_table();
    for (key, value) in e_keys.unwrap() {
        println!("Key: {}, Value: {}", key, value);
    }
    Ok(())
}

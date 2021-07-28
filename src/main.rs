extern crate clap;
use clap::{Arg, App};

use std::fs;

fn main() { 
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
        _ => println!("Show help message"),
    }

    let filename = matches.value_of("filename").unwrap();
    println!("Using keyboard layout file: {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Unable to read keyboard layout file");
    println!("With text:\n{}", contents);
}

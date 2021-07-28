extern crate clap;
use clap::{Arg, App};

fn main() { 
    let matches =
        App::new("Unikey") 
        .version("0.1.0")
        .author("Joseph Diza <josephm.diza@gmail.com>")
        .about("Create linux xkb keyboard layouts")
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .multiple(true)
            .help("Show verbose output"))
        .get_matches();

    match matches.occurrences_of("v") {
        0 => println!("Show verbose output"),
        _ => println!("Show help message"),
    }
}

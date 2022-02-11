use clap::{Arg, App, AppSettings};
use toml::Value;
use failure::Error;
use utils::files::{read_file, write_file};
use utils::layout::{Layout, create_layout, create_evdev, create_lst};

fn main() -> Result<(), Error> {
    let app = App::new("Unikey")
        .version("0.1.1")
        .author("Joseph Diza <josephm.diza@gmail.com>")
        .about("Create linux xkb keyboard layouts")
        .arg(Arg::new("v").help("Show verbose output"))
        .arg(Arg::new("keyboard_layout")
            .help("Specify the file path to the keyboard layout config"))
        .arg(Arg::new("name")
            .help("Specify the name of your keyboard layout"))
        .arg(Arg::new("desc")
            .help("Give a brief description for keyboard layout name. Ex. English (US)"));

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
    let config = &keyboard_layout["config"];

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

    let layout: Layout = Layout::new(keyboard_name, keyboard_desc);
    let rendered_layout = create_layout(&keyboard_layout, &layout, verbose);
    let evdev = create_evdev(&layout);
    let base_lst = create_lst(&layout, "base.lst");
    let evdev_lst = create_lst(&layout, "evdev.lst");
    write_file(&rendered_layout, keyboard_name);
    write_file(&evdev, "evdev.xml");
    write_file(&base_lst, "base.lst");
    write_file(&evdev_lst, "evdev.lst");
    
    Ok(())
}

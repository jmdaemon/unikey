// Third Party Crates
use log::{debug, info};
use clap::{Arg, Command};
use tera::{Tera, Context};
use toml::Value;
use utility::layout::{Keys, init_tera};

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

const ROWS: [&str; 4] = ["e", "d", "c", "b"];

/// Returns a HashMap of keys from a keyboard layout
pub fn parse_rows(kb_layout: &Value) -> HashMap<&str, Keys> {
    let mut row_keys: HashMap<&str, Keys> = HashMap::new();

    // For every row in the row of keys
    for row_name in ROWS {

        // Get the current row
        let row = &kb_layout["rows"][&row_name];

        // Get all the keys values for that row
        let key_values = row.as_array().unwrap();

        // Convert the key values to a vector of strings
        let mut keys: Vec<String> = vec![];
        for key in key_values {
            keys.push(key.to_string());
        }
        // Add the keys for the row
        row_keys.insert(row_name, Keys { keys: keys });
    }
    row_keys
}

pub fn parse_misc(kb_layout: &Value) -> HashMap<&str, String> {
    let mut row_misc: HashMap<&str, String> = HashMap::new();
    let kb_misc = &kb_layout["rows"]["misc"];
    row_misc.insert("BKSL", kb_misc["BKSL"].to_string());
    row_misc.insert("TLDE", kb_misc["TLDE"].to_string());
    row_misc
}

/// Display keyboard config debug info
pub fn show_kb_layout(kb_name: &str, kb_desc: &str, kb_layout_contents: &str) {
    debug!("Keyboard Config\n");
    debug!("Keyboard Name       : {}\n", kb_name);
    debug!("Keyboard Descrption : {}\n", kb_desc);
    debug!("{}", "=".repeat(16));

    debug!("Keyboard Config Contents: ");
    debug!("{}", "=".repeat(16));
    debug!("{}\n", kb_layout_contents);
    debug!("{}", "=".repeat(16));
}

#[derive(Default, Debug)]
pub struct KeyboardLayout<'a, 'b> {
    pub kb_name: &'b str,
    pub kb_desc: &'b str,
    pub kb_rows: HashMap<&'a str, Keys>,
    pub kb_misc: HashMap<&'a str, String>
}

impl KeyboardLayout <'static, 'static> {
    pub fn new<'a, 'b> (
            kb_name: &'b str,
            kb_desc: &'b str,
            kb_rows: HashMap<&'a str, Keys>,
            kb_misc: HashMap<&'a str, String>) -> KeyboardLayout<'a, 'b> {
        KeyboardLayout { kb_name, kb_desc, kb_rows, kb_misc }
    }
}

/// Populate the Tera Context with the row key values
pub fn populate_row_keys(context: &mut Context, kb: &KeyboardLayout) {
    for (row, keys) in &kb.kb_rows {
        info!("Row: {}", row);

        let mut index = 1;
        let keys = &keys.keys;
        for keyval in keys {
            info!("Key value: {}", keyval);
            let key_index = format!("{}_key_{}", row, index);
            context.insert(key_index, &keyval);
            index += 1;
        }
    }
}

/// Populate the Tera Context with the misc key values
pub fn populate_misc_keys(context: &mut Context, kb: &KeyboardLayout) {
    let kb_misc = &kb.kb_misc;
    for (key, val) in kb_misc.iter() {
        let key_index = format!("misc_{}", key);
        info!("Key Name: {}", key_index);
        info!("Key Value: {}", val);
        context.insert(key_index, val);
    }
}

/// Populate the Tera Context with the keyboard name and description values
pub fn populate_context(kb: &KeyboardLayout) -> Context {
    let mut context = Context::new();
    context.insert("layout_name", &kb.kb_name);
    context.insert("layout_desc", &kb.kb_desc);
    context
}

/// Renders the template with the Tera Context to a string
pub fn render_template(tera: &Tera, template: &str, context: &mut Context) -> String {
    let rendered = tera.render(template, &context).expect("Template failed to render");
    rendered.to_string()
}

/// Format the rendered content to a string
pub fn format_rendered(title: &str, rendered: &str) -> String {
    let mut result = "".to_owned();
    result.push_str(&format!("{}\n", title));
    result.push_str(&format!("{}\n", "=".repeat(16)));
    result.push_str(&format!("{}\n", rendered));
    result.push_str(&format!("{}", "=".repeat(16)));
    result
}

/// Displays the rendered template
pub fn show_rendered(dryrun: bool, title: &str, rendered: &str) {
    let format_rendered = format_rendered(title, &rendered.to_string());
    if dryrun {
        println!("{}", format_rendered);
    } else {
        info!("{}", format_rendered);
    }
}

/// Populate evdev.xml, base.lst, evdev.lst templates
pub fn populate_linux_kb_definition(kb: &KeyboardLayout, dryrun: bool, tera: &Tera, template: &str) -> String {
    let mut context = populate_context(&kb);
    let rendered_template = render_template(&tera, template, &mut context);
    show_rendered(dryrun, "Linux Keyboard Definition", &rendered_template);
    rendered_template.to_string()
}

//pub fn populate_linux_kb<'a> (kb: &'a KeyboardLayout, dryrun: bool, tera: &Tera) -> HashMap<String, &'static str> {
pub fn populate_linux_kb(kb: &KeyboardLayout, dryrun: bool, tera: &Tera) -> HashMap<String, String> {
    // Populate the layout template
    let mut layout_context = populate_context(&kb);
    populate_row_keys(&mut layout_context, &kb);
    populate_misc_keys(&mut layout_context, &kb);

    let rendered_layout = render_template(&tera, "layout.tmpl", &mut layout_context);
    show_rendered(dryrun, "Linux Keyboard Layout", &rendered_layout);

    let rendered_evdev = populate_linux_kb_definition(&kb, dryrun, &tera, "evdev.xml.tmpl");
    let rendered_base_lst = populate_linux_kb_definition(&kb, dryrun, &tera, "base.lst.tmpl");
    let rendered_evdev_lst = populate_linux_kb_definition(&kb, dryrun, &tera, "evdev.lst.tmpl");

    // Create a hashmap containing the output file name, and its rendered contents
    let mut rendered: HashMap<String, String> = HashMap::new();
    //rendered.insert(&kb.kb_name, &rendered_layout);
    rendered.insert(kb.kb_name.to_string(), rendered_layout);
    rendered.insert("evdev.xml".to_string(), rendered_evdev);
    rendered.insert("base.lst".to_string(), rendered_base_lst);
    rendered.insert("evdev.lst".to_string(), rendered_evdev_lst);
    rendered
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
    let tera = init_tera();

    // Populate templates with values from keyboard config
    let rendered = populate_linux_kb(&kb, dryrun, &tera);

    if dryrun {
        // Exit early after printing the template
        exit(0); 
    }



    println!("Writing rendered templates to {}", kb_output_fp);

    // Create directory if it doesn't exist
    create_dir_all(kb_output_fp).expect(&format!("Directory {} could not be created", kb_output_fp));
    for (filename, contents) in rendered {
        let fp = format!("{}/{}", kb_output_fp, filename);
        // Write the template to the output folder
        write(fp, contents).expect("Unable to write file");
    }
}

use log::{debug, error, info, warn};
use clap::{Arg, Command};
use toml::Value;
use failure::Error;
use std::fs::read_to_string;
use std::collections::HashMap;
use utility::files::{write_file};
use tera::{Tera, Context};
use utility::layout::{Keys, KeyMap, Layout, create_layout, create_evdev, create_lst, init_tera};
use std::process::exit;

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
            .help("Don't output files to disk"));
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
    result.push_str(&format!("Rendered Template: \n"));
    result.push_str(&format!("{}\n", title));
    result.push_str(&format!("{}\n", "=".repeat(16)));
    result.push_str(&format!("{}\n", rendered));
    result.push_str(&format!("{}", "=".repeat(16)));
    result
}

/// Displays the rendered template and exits if dryrun was specified
pub fn show_rendered(dryrun: bool, title: &str, rendered: &str) {
    let format_rendered = format_rendered(title, &rendered.to_string());
    if dryrun {
        println!("{}", format_rendered);
        exit(0); // Exit early after printing the template
    } else {
        info!("{}", format_rendered);
    }
}

fn main() -> Result<(), Error> {
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
    // Populate the layout template
    // Initialize the template context
    let mut layout_context = populate_context(&kb);
    populate_row_keys(&mut layout_context, &kb);
    populate_misc_keys(&mut layout_context, &kb);

    let rendered_layout = render_template(&tera, "layout.tmpl", &mut layout_context);
    show_rendered(dryrun, "Linux Keyboard Layout", &rendered_layout);

    // Populate the evdev template
    let mut evdev_context = populate_context(&kb);
    let rendered_evdev = render_template(&tera, "evdev.xml.tmpl", &mut evdev_context);

    // Populate the base.lst template
    let mut base_lst_context = populate_context(&kb);
    let rendered_base_lst = render_template(&tera, "base.lst.tmpl", &mut base_lst_context);

    // Populate the evdev.lst template
    let mut evdev_lst_context = populate_context(&kb);
    let rendered_evdev_lst = render_template(&tera, "evdev.lst.tmpl", &mut evdev_lst_context);

    // Store rendered templates in vector
    // For every rendered template
    // Write template to output folder

    // Initializes Tera templates
    let layout: Layout = Layout::new(kb_name, kb_desc);
    let ekeys = &kb_layout["rows"]["e"].as_array();
    for key in ekeys.unwrap().iter() {
        //let skey = key.to_string();
        //println!("{}", skey)
        println!("Key: {}", key)
    }

    let kmap: KeyMap = KeyMap::new(kb_layout);
    //println!("{:?}", kmap.keys.keys);
    for rows in kmap.keys.iter() {
        for keys in rows.keys.iter() {
            println!("{}", keys.to_string());
        }
    }

    exit(0);
    let rendered_layout = create_layout(&kb_layout, &layout, false);

    // Render our variables into the templates
    let evdev = create_evdev(&layout);
    let base_lst = create_lst(&layout, "base.lst");
    let evdev_lst = create_lst(&layout, "evdev.lst");

    // Write Linux XKB templates to layouts output directory
    write_file(&rendered_layout, kb_name);
    write_file(&evdev, "evdev.xml");
    write_file(&base_lst, "base.lst");
    write_file(&evdev_lst, "evdev.lst");
    
    Ok(())
}

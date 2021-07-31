pub mod files {
    use std::fs;
    use std::fs::File;
    use std::io::Write;

    pub fn read_file(filename: &str) -> String {
        return fs::read_to_string(filename)
            .expect("Unable to read keyboard layout file").to_owned();
    }

    pub fn write_file(keyboard_layout: String, filename: &str) {
        let mut f = File::create(["layouts", filename].join("/")).expect("Unable to create file");
        f.write_all(keyboard_layout.as_bytes()).expect("Unable to write data");
    }

}


pub mod layout {
    use toml::{Value, value::Table};
    use tera::{Tera, Context};
    use std::collections::HashMap;
    use std::process::exit;

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

    pub fn create_layout(keyboard_layout: Value, layout_name: String, layout_desc: String, verbose: bool) -> String {
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

    pub fn create_evdev(layout_name: String, layout_desc: String) -> String {
        let tera = match Tera::new("templates/**/*.tmpl") {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Parsing error(s): {}", e);
                exit(1);
            }
        };
        let mut context = Context::new();
        context.insert("layout_name", &layout_name);
        context.insert("layout_desc", &layout_desc);

        println!("=== Linux Keyboard Definition ===");
        let rendered = tera.render("evdev.xml.tmpl", &context).expect("Template failed to render");
        println!("{}", rendered);
        return rendered;
    }

    pub fn create_lst(layout_name: String, layout_desc: String, lst_name: &str) -> String {
        let tera = match Tera::new("templates/**/*.tmpl") {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Parsing error(s): {}", e);
                exit(1);
            }
        };
        let mut context = Context::new();
        context.insert("layout_name", &layout_name);
        context.insert("layout_desc", &layout_desc);

        println!("=== {} ===", lst_name);
        let rendered = tera.render(&[lst_name,".tmpl"].join(""), &context).expect("Template failed to render");
        println!("{}", rendered);
        return rendered;
    }

}

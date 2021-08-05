pub mod files {
    use std::fs;
    use std::fs::File;
    use std::io::Write;

    pub fn read_file(filename: &str) -> String {
        return fs::read_to_string(filename)
            .expect("Unable to read keyboard layout file").to_owned();
    }

    pub fn write_file(keyboard_layout: &str, filename: &str) {
        let mut f = File::create(["layouts", filename].join("/")).expect("Unable to create file");
        f.write_all(keyboard_layout.as_bytes()).expect("Unable to write data");
    }

}


pub mod layout {
    use toml::{Value, value::Table};
    use tera::{Tera, Context};
    use std::collections::HashMap;
    use std::process::exit;

    pub struct Layout {
        pub name: String,
        pub desc: String
    }

    impl Layout {
        pub fn new<S: Into<String>>(name: S, desc: S) ->Layout {
            Layout { name: name.into(), desc: desc.into() }
        }
    }

    fn init_tera() -> Tera {
        let tera = match Tera::new("templates/**/*.tmpl") {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Parsing error(s): {}", e);
                exit(1);
            }
        };
        return tera;
    }

    fn init_context(rows_table: HashMap<&str, Option<&Table>>, layout: &Layout, verbose: bool) -> tera::Context {
        let mut context = Context::new();
        context.insert("layout_name", &layout.name);
        context.insert("layout_desc", &layout.desc);
        if verbose {
            println!("Using layout name: {}", &layout.name);
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

    //fn create_def(layout_name: String, layout_desc: String, msg: &str, output: &str) -> String {
    fn create_def(layout: &Layout, msg: &str, output: &str) -> String {
        let tera = init_tera();
        let mut context = Context::new();
        context.insert("layout_name", &layout.name);
        context.insert("layout_desc", &layout.desc);

        println!("=== {} ===", msg);
        let rendered = tera.render(output, &context).expect("Template failed to render");
        println!("{}", rendered);
        return rendered;
    }

    pub fn create_layout(keyboard_layout: Value, layout: &Layout, verbose: bool) -> String {
        let rows = ["e", "d", "c", "b", "misc"];
        let mut rows_table = HashMap::new();

        for row in rows { rows_table.insert(row, keyboard_layout["rows"][&row].as_table()); };
        let tera = init_tera();

        let context = init_context(rows_table, layout, verbose);
        println!("=== Linux Keyboard Layout ===");
        let rendered = tera.render("layout.tmpl", &context).expect("Template failed to render");
        println!("{}", rendered);
        return rendered;
    }

    pub fn create_evdev(layout: &Layout) -> String {
        return create_def(layout, "Linux Keyboard Definition", "evdev.xml.tmpl");
    }

    pub fn create_lst(layout: &Layout, lst_name: &str) -> String {
        return create_def(layout, lst_name, &[lst_name,".tmpl"].join(""));
    }
}

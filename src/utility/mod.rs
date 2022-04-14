pub mod files {
    /**
    * Utilities for reading and writing to files
    */
    use std::fs::File;
    use std::io::Write;

    /// Writes a keyboard layout to disk
    /// This creates the layouts directory.
    /// Note that this directory will be customizable in the future.
    pub fn write_file(keyboard_layout: &str, filename: &str) {
        let mut f = File::create(["layouts", filename].join("/")).expect("Unable to create file");
        f.write_all(keyboard_layout.as_bytes()).expect("Unable to write data");
    }

}


pub mod layout {
    /**
    * Manage Linux XKB layouts
    */
    use toml::{Value, value::Table};
    use tera::{Tera, Context};
    use std::collections::HashMap;
    use std::process::exit;

    #[derive(Default, Debug)]
    pub struct Keys {
        //pub keys: Vec<Value>
        pub keys: Vec<String>
    }

    impl Keys {
        //pub fn new(row_keys: Vec<Value>) -> Keys {
        pub fn new(row_keys: Vec<String>) -> Keys {
            Keys { keys: row_keys }
        }
    }

    pub struct KeyMap {
        pub keys: Vec<Keys>
    }

    impl KeyMap {
        pub fn new(keyboard_layout: Value) -> KeyMap {
            let mut row_keys: Vec<Keys> = vec![];
            //let rows = ["e", "d", "c", "b", "misc"];
            let rows = ["e", "d", "c", "b"];
            for row in rows {
                let currow = &keyboard_layout["rows"][&row];
                let keyvec = currow.as_array().unwrap();
                let mut vecstr: Vec<String> = vec![];
                for key in keyvec { vecstr.push(key.to_string()); }
                //&row_keys.push(Keys { keys: currow.as_array().unwrap().to_vec() });
                //&row_keys.push(Keys { keys: currow.as_array().unwrap().to_vec() });
                row_keys.push(Keys { keys: vecstr });
            }
        KeyMap { keys: row_keys }
        }
    }


    pub struct Layout {
        pub name: String,
        pub desc: String
    }

    impl Layout {
        // Pass in either &str, or String
        pub fn new<S: Into<String>>(name: S, desc: S) -> Layout {
            Layout { name: name.into(), desc: desc.into() }
        }
    }

    /// Creates a Tera instance with all the templates found in templates
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

    /// Renders the template with the given variables
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

    pub fn create_layout(keyboard_layout: &Value, layout: &Layout, verbose: bool) -> String {
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

    /// Renders the keyboard layout into the template
    fn create_def(layout: &Layout, msg: &str, output: &str) -> String {
        // Note that this is created multiple times, this should only
        // be created statically once.
        let tera = init_tera();
        let mut context = Context::new();
        context.insert("layout_name", &layout.name);
        context.insert("layout_desc", &layout.desc);

        println!("=== {} ===", msg);
        let rendered = tera.render(output, &context).expect("Template failed to render");
        println!("{}", rendered);
        return rendered;
    }

    /// Returns the rendered evdev.xml, base.xml files
    pub fn create_evdev(layout: &Layout) -> String {
        return create_def(layout, "Linux Keyboard Definition", "evdev.xml.tmpl");
    }

    /// Returns the rendered evdev.lst, base.lst files
    pub fn create_lst(layout: &Layout, lst_name: &str) -> String {
        return create_def(layout, lst_name, &[lst_name,".tmpl"].join(""));
    }
}

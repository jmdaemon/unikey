// Third Party Crates
use log::debug;
use std::collections::HashMap;

pub const ROWS: [&str; 4] = ["e", "d", "c", "b"];

#[derive(Default, Debug)]
pub struct Keys {
    pub keys: Vec<String>
}

impl Keys {
    pub fn new(row_keys: Vec<String>) -> Keys {
        Keys { keys: row_keys }
    }
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

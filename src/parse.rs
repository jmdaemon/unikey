// Unikey Modules
use crate::keyboard::{Keys, ROWS};

// Third Party Crates
use toml::Value;

// Standard Library
use std::collections::HashMap;

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

/// Parse a HashMap of the misc keys from a keyboard layout
pub fn parse_misc(kb_layout: &Value) -> HashMap<&str, String> {
    let mut row_misc: HashMap<&str, String> = HashMap::new();
    let kb_misc = &kb_layout["rows"]["misc"];
    row_misc.insert("BKSL", kb_misc["BKSL"].to_string());
    row_misc.insert("TLDE", kb_misc["TLDE"].to_string());
    row_misc
}


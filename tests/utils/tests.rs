fn main() {
}

#[cfg(test)]
mod tests {
    use toml::Value;
    use failure::Error;
    use std::fs::read_to_string;

    #[test]
    fn can_read_to_strings() {
        assert_ne!(String::new().is_empty(), read_to_string("key.layout.toml").is_empty());
    }

    #[test]
    fn keyboard_layout_is_formatted_correctly() -> Result<(), Error> {
        let filename = "key.layout.toml";
        let contents = read_to_string(filename);
        let keyboard_layout: Value = toml::from_str(&contents)?;

        let ekeys : Vec<Value> = keyboard_layout["rows"]["e"].as_array().unwrap().to_vec();
        let key_1 : String = ekeys[0].to_string();
        //let key_1 : String = keyboard_layout["rows"]["e"]["key-1"].to_string();
        assert_eq!(false, key_1.is_empty());
        //assert_eq!(&keyboard_layout["rows"]["e"]["key-1"].as_str(), Some("1"));
        assert_eq!(key_1.as_str(), "\"1\"");
        Ok(())
    }
}

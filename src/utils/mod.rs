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

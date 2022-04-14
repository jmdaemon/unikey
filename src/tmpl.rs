// Unikey Modules
use crate::keyboard::KeyboardLayout;

// Third Party Crates
use log::{info, error};
use tera::{Tera, Context};

// Standard Library
use std::process::exit;

/// Initialize the tera context
pub fn init_tera(kb_type: &str) -> Tera {
    let tera = match Tera::new(&format!("templates/{}/**/*.tmpl", kb_type)) {
        Ok(t) => t,
        Err(e) => {
            error!("Parsing error(s): {}", e);
            exit(1);
        }
    };
    return tera;
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

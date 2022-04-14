// Unikey Modules
use crate::keyboard::KeyboardLayout;
use crate::tmpl::{show_rendered, render_template, populate_context, populate_row_keys, populate_misc_keys};

// Third Party Crates
use tera::{Tera};

// Standard Library
use std::collections::HashMap;

/// Populate evdev.xml, base.lst, evdev.lst templates
pub fn populate_linux_kb_definition(kb: &KeyboardLayout, dryrun: bool, tera: &Tera, template: &str) -> String {
    let mut context = populate_context(&kb);
    let rendered_template = render_template(&tera, template, &mut context);
    show_rendered(dryrun, "Linux Keyboard Definition", &rendered_template);
    rendered_template.to_string()
}

/// Output a hashmap containing the files for a custom linux keyboard mapping
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

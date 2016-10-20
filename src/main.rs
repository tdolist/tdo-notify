#[macro_use]
#[allow(dead_code, unused_imports)]
extern crate json;
use json::{parse, JsonValue};
use std::io::Read;
use std::fs::File;
use std::env;
mod mailing;
mod settings;


fn main() {
    let mut content = String::new();
    let home = match env::home_dir() {
        Some(path) => path.to_str().unwrap().to_owned() + "/.tdo/",
        None => panic!("No Path found"),
    };
    let gotten_settings = settings::Settings::load(&home);
    let mut serversettings: settings::Settings;
    if gotten_settings.is_err() {
        let inputs = settings::get_input();
        serversettings =
            settings::Settings::new(inputs.0, inputs.1, inputs.2, inputs.3, inputs.4, inputs.5);
        serversettings = serversettings.store(&home);
    } else {
        serversettings = gotten_settings.unwrap();
    }
    println!("{:?}", serversettings);
    let tdo_json = home + "list.json";
    let tdos = get_json(tdo_json);
    for list in tdos.entries() {
        content = check_undone(list, &content);
    }
    let mail = mailing::gen_mail(&content);
    // println!("{:?}", mail);
}

fn get_json(tdo_json: String) -> json::JsonValue {
    let mut json_file = File::open(tdo_json).unwrap();
    let mut json_data = String::new();
    json_file.read_to_string(&mut json_data).unwrap();
    parse(&json_data).unwrap()
}

fn check_undone(list: (&str, &JsonValue), content: &String) -> String {
    let mut intern: String = content.to_owned();
    let mut has_tasks = false;
    intern.push_str(list.0);
    intern.push_str("\n------------------------------\n");
    for item in list.1.entries() {
        let mut tdo_content = item.1.members();
        let text = tdo_content.next().unwrap().as_str().unwrap();
        let done = tdo_content.next().unwrap().as_bool().unwrap();
        if !done {
            has_tasks = true;
            intern.push_str(text);
            intern.push_str("\n");
        }
    }
    if !has_tasks {
        intern.push_str("This Category has no (undone) Tasks\n");
    }
    intern
}

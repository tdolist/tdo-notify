#[macro_use]
extern crate json;
extern crate lettre;
extern crate libc;
use std::env;
mod mailing;
mod settings;
mod checking;


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
    let tdo_json = home + "list.json";
    let tdos = settings::get_json(tdo_json);
    for list in tdos.entries() {
        content = checking::check_undone(list, content);
    }
    let mail = mailing::gen_mail(&content, serversettings.mailfrom, serversettings.mailto);
    mailing::send_mail(mail,
                       serversettings.server,
                       serversettings.user,
                       serversettings.pass,
                       serversettings.port);
}

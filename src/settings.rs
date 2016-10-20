use json::{from, parse, JsonValue};
use std::io;
use std::io::{Error, ErrorKind, Read};
use std::fs::File;

#[derive(Debug, Clone)]
pub struct Settings {
    pub mailto: String,
    pub mailfrom: String,
    pub server: String,
    pub user: String,
    pub pass: String,
    pub port: u16,
}

impl Into<JsonValue> for Settings {
    fn into(self) -> JsonValue {
        object!{
            "mailto" => self.mailto,
            "mailfrom" => self.mailfrom,
            "server" => self.server,
            "user" => self.user,
            "pass" => self.pass,
            "port" => self.port

        }
    }
}

impl Settings {
    pub fn new(mailto: String,
               mailfrom: String,
               server: String,
               user: String,
               pass: String,
               port: u16)
               -> Settings {
        Settings {
            mailto: mailto,
            mailfrom: mailfrom,
            server: server,
            user: user,
            pass: pass,
            port: port,
        }
    }
    pub fn load(path: &String) -> Result<Settings, Error> {
        let file = File::open(path.to_owned() + "notify.json");
        if file.is_ok() {
            Ok({
                let mut data = String::new();
                file.unwrap().read_to_string(&mut data).unwrap();
                let parsed = parse(&data).unwrap();
                Settings {
                    mailto: parsed["mailto"].as_str().unwrap().to_string(),
                    mailfrom: parsed["mailfrom"].as_str().unwrap().to_string(),
                    server: parsed["server"].as_str().unwrap().to_string(),
                    user: parsed["user"].as_str().unwrap().to_string(),
                    pass: parsed["pass"].as_str().unwrap().to_string(),
                    port: parsed["port"].as_u16().unwrap(),
                }
            })
        } else {
            Err(Error::new(ErrorKind::NotFound, "File not found."))
        }


    }
    pub fn store(self, path: &String) -> Settings {
        let mut file = File::create(path.to_owned() + "notify.json").unwrap();
        let save = from(self.clone());
        let _ = save.write_pretty(&mut file, 4);
        self
    }
}

pub fn get_json(tdo_json: String) -> JsonValue {
    let mut json_file = File::open(tdo_json).unwrap();
    let mut json_data = String::new();
    json_file.read_to_string(&mut json_data).unwrap();
    parse(&json_data).unwrap()
}


pub fn get_input() -> (String, String, String, String, String, u16) {
    println!("Mailto: ", );
    let mut mailto = String::new();
    io::stdin()
        .read_line(&mut mailto)
        .expect("Failed to read line");
    mailto.pop();

    println!("Mailfrom: ", );
    let mut mailfrom = String::new();
    io::stdin()
        .read_line(&mut mailfrom)
        .expect("Failed to read line");
    mailfrom.pop();

    println!("Server: ", );
    let mut server = String::new();
    io::stdin()
        .read_line(&mut server)
        .expect("Failed to read line");
    server.pop();

    println!("User: ", );
    let mut user = String::new();
    io::stdin()
        .read_line(&mut user)
        .expect("Failed to read line");
    user.pop();

    println!("Password: ", );
    let mut pass = String::new();
    io::stdin()
        .read_line(&mut pass)
        .expect("Failed to read line");
    pass.pop();

    println!("Port: ", );
    let mut port_str = String::new();
    io::stdin()
        .read_line(&mut port_str)
        .expect("Failed to read line");
    port_str.pop();
    (mailto, mailfrom, server, user, pass, port_str.parse::<u16>().unwrap())
}

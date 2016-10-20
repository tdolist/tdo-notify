extern crate json;
use json::{parse, JsonValue};
use std::io;
use std::io::{Error, ErrorKind, Read};
use std::fs::File;

#[derive(Debug, Clone)]
pub struct Settings {
    mailto: String,
    mailfrom: String,
    server: String,
    user: String,
    pass: String,
    port: u32,
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
               port: u32)
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
                    port: parsed["port"].as_u32().unwrap(),
                }
            })
        } else {
            Err(Error::new(ErrorKind::NotFound, "File not found."))
        }


    }
    pub fn store(self, path: &String) -> Settings {
        println!("Try to store", );
        let mut file = File::create(path.to_owned() + "notify.json").unwrap();
        let save = json::from(self.clone());
        save.write_pretty(&mut file, 4);
        self
    }
}

pub fn get_input() -> (String, String, String, String, String, u32) {
    println!("Mailto: ", );
    let mut mailto = String::new();
    io::stdin()
        .read_line(&mut mailto)
        .expect("Failed to read line");

    println!("Mailfrom: ", );

    let mut mailfrom = String::new();
    io::stdin()
        .read_line(&mut mailfrom)
        .expect("Failed to read line");
    println!("Server: ", );

    let mut server = String::new();
    io::stdin()
        .read_line(&mut server)
        .expect("Failed to read line");
    println!("User: ", );

    let mut user = String::new();
    io::stdin()
        .read_line(&mut user)
        .expect("Failed to read line");
    println!("Password: ", );
    let mut pass = String::new();
    io::stdin()
        .read_line(&mut pass)
        .expect("Failed to read line");
    println!("Port: ", );
    let mut port_str = String::new();
    io::stdin()
        .read_line(&mut port_str)
        .expect("Failed to read line");
    (mailto, mailfrom, server, user, pass, port_str.parse::<u32>().unwrap())
}
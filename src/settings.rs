use base64::{encode, decode};
use tdo_core::error::StorageError;
use std::fs::File;
use std::io::{Read, Write};



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub name: String,
    pub mailto: String,
    pub mailfrom: String,
    pub server: String,
    pub user: String,
    pub pass: String,
    pub port: u16,
}

impl Settings {
    pub fn new(name:String,
               mailto: String,
               mailfrom: String,
               server: String,
               user: String,
               pass: String,
               port: u16)
               -> Settings {
        Settings {
            name: name,
            mailto: mailto,
            mailfrom: mailfrom,
            server: server,
            user: user,
            pass: pass,
            port: port,
        }
    }

    pub fn load(path: &String) -> Result<Settings, StorageError> {
        match File::open(path.to_owned() + ".notify") {
            Ok(mut file) => {
                let mut encoded = String::new();
                file.read_to_string(&mut encoded).unwrap();
                let raw = String::from_utf8(decode(&encoded).unwrap()).unwrap();
                match super::serde_json::from_str(&raw) {
                    Ok(settings) => Ok(settings),
                    Err(_) => Err(StorageError::FileCorrupted),
                }
            }
            Err(_) => Err(StorageError::FileNotFound),
        }
    }

    pub fn store(&self, path: &String) -> Result<(), StorageError> {
        match File::create(path.to_owned() + ".notify") {
            Ok(mut file) => {
                let raw = super::serde_json::to_string(self).unwrap().into_bytes();
                match file.write(&encode(&raw[..]).into_bytes()) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(StorageError::SaveFailure),
                }
            }
            Err(_) => Err(StorageError::SaveFailure),
        }
    }
}

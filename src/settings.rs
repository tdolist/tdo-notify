//! Settings needed for mail notification.
use base64::{encode, decode};
use super::tdo_core::error::*;
use std::fs::File;
use std::io::{Read, Write};


/// Base stuct for Mail settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// Receipiant name
    pub name: String,
    /// Receipiant mail address
    pub mailto: String,
    /// sender mail address
    pub mailfrom: String,
    /// Server URL/IP
    pub server: String,
    /// Username at the mail server
    pub user: String,
    /// Password to authenticate
    pub pass: String,
    /// SMTP-Port
    pub port: u16,
}

impl Settings {
    /// Creates a new settings object.
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

    /// Loads a Settings object from JSON.
    pub fn load(path: &String) -> TdoResult<Settings> {
        match File::open(path.to_owned() + ".notify") {
            Ok(mut file) => {
                let mut encoded = String::new();
                file.read_to_string(&mut encoded).unwrap();
                let raw = String::from_utf8(decode(&encoded).unwrap()).unwrap();
                match super::serde_json::from_str(&raw) {
                    Ok(settings) => Ok(settings),
                    Err(_) => Err(ErrorKind::StorageError(storage_error::ErrorKind::FileCorrupted).into()),
                }
            }
            Err(_) => Err(ErrorKind::StorageError(storage_error::ErrorKind::FileNotFound).into())
        }
    }

    /// Stores a Settings object to JSON
    pub fn store(&self, path: &String) -> TdoResult<()> {
        match File::create(path.to_owned() + ".notify") {
            Ok(mut file) => {
                let raw = super::serde_json::to_string(self).unwrap().into_bytes();
                match file.write(&encode(&raw[..]).into_bytes()) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(ErrorKind::StorageError(storage_error::ErrorKind::SaveFailure).into()),
                }
            }
            Err(_) => Err(ErrorKind::StorageError(storage_error::ErrorKind::SaveFailure).into()),
        }
    }
}

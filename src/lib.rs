//! The notification module for Tdo server.
#![deny(missing_docs, unsafe_code,
        missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unused_import_braces, unused_qualifications)]
#![warn(missing_debug_implementations)]
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tdo_core;
extern crate lettre;
extern crate base64;
extern crate libc;

use tdo_core::list;
pub mod settings;
pub mod util;
pub mod mail;

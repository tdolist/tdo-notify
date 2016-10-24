extern crate libc;
use std::{slice, io, ptr};
use lettre::transport::{smtp, EmailTransport};
use lettre::email::{Email, EmailBuilder};

pub fn gen_mail(content: &String, mailto: String, mailfrom: String) -> Email {
    let mail = EmailBuilder::new()
        .to(mailfrom.as_str())
        .from((mailto.as_str(), "tdo notify"))
        .body(&gen_body(content))
        .subject("Your undone taks")
        .build()
        .unwrap();
    mail
}

pub fn send_mail(mail: Email, server: String, user: String, pass: String, port: u16) -> bool {
    let mut sender = smtp::SmtpTransportBuilder::new((server.as_str(), port))
        .unwrap()
        .credentials(&user, &pass)
        .security_level(smtp::SecurityLevel::AlwaysEncrypt)
        .smtp_utf8(true)
        .connection_reuse(true)
        .build();
    let result = sender.send(mail);
    result.is_ok()
}

fn gen_body(content: &String) -> String {
    let mut head: String = format!("Hello {},\nhere are your undone tasks.\n\n\n", get_user_name().unwrap());
    head.push_str(content);
    head
}

fn get_user_name() -> Result<String, io::Error> {
    unsafe {
        let uid = libc::geteuid();
        let user = ptr::read(libc::getpwuid(uid));
        let name = String::from_utf8_unchecked(slice::from_raw_parts(user.pw_gecos as *const u8,
                                                           libc::strlen(user.pw_gecos) as usize)
                .to_vec());
        if name == "" {
            Err(io::Error::last_os_error())
        } else {
            Ok(name)
        }
    }
}

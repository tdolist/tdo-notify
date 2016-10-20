use lettre::transport::{smtp, EmailTransport};
use lettre::email::{Email, EmailBuilder};

pub fn gen_mail(content: &String, mailto: String, mailfrom: String) -> Email {
    let mail = EmailBuilder::new()
        .to(&mailfrom[..])
        .from((&mailto[..], "tdo notify"))
        .body(content)
        .subject("Your undone taks")
        .build()
        .unwrap();
    mail
}

pub fn send_mail(mail: Email, server: String, user: String, pass: String, port: u16) -> bool {
    let mut sender = smtp::SmtpTransportBuilder::new((&server[..], port))
        .unwrap()
        .credentials(&user, &pass)
        .security_level(smtp::SecurityLevel::AlwaysEncrypt)
        .smtp_utf8(true)
        .connection_reuse(true)
        .build();

    let result = sender.send(mail);
    println!("{:?}", result.is_ok());
    result.is_ok()
}

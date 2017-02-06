use lettre::transport::EmailTransport;
use lettre::transport::smtp::{error, SecurityLevel, SmtpTransportBuilder};
use lettre::email::{Email, EmailBuilder};
use settings::Settings;
use util::gen_tasks;

pub fn gen_mail(tdo: &super::list::Tdo, settings: &Settings) -> Email {
    EmailBuilder::new()
        .to(settings.mailto.as_str())
        .from((settings.mailfrom.as_str(), "tdo notify"))
        .body(&gen_body(tdo, &settings.name))
        .subject("Your undone taks")
        .build()
        .unwrap()
}

pub fn send_mail(mail: Email, settings: &Settings) -> Result<(), error::Error> {
    let mut sender = SmtpTransportBuilder::new((settings.server.as_str(), settings.port))
        .unwrap()
        .credentials(&settings.user, &settings.pass)
        .security_level(SecurityLevel::AlwaysEncrypt)
        .smtp_utf8(true)
        .connection_reuse(true)
        .build();
    match sender.send(mail) {
        Ok(_) => Ok(()),
        Err(x) => Err(x),
    }
}

pub fn gen_body(tdo: &super::list::Tdo, name: &str) -> String {
    match gen_tasks(tdo) {
        Some(x) => {
            let mut tasks = format!("Hello {},\n\nhere are your undone tasks.\n\n\n", name);
            tasks.push_str(&x);
            tasks
        }
        None => {
            format!("Hello {},\n\nCongratulation, you have no undone tasks! :-)",
                    name)
        }
    }
}

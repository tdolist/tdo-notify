extern crate lettre;

use self::lettre::email::{Email,EmailBuilder};

pub fn gen_mail(content: &String) -> Email {
    let mail = EmailBuilder::new()
    .to("mail@felixdoering.com")
    .from("Doering.Felix@googlemail.com")
    .body(content)
    .subject("tdo notify")
    .build()
    .unwrap();
    mail
}

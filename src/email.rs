extern crate lettre;
extern crate lettre_email;

use lettre_email::EmailBuilder;
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::extension::ClientId;
use lettre::EmailTransport;
use lettre::SmtpTransport;
use lettre::smtp::ConnectionReuseParameters;

pub fn send_email(domain_valid: &str) {
    
    let send_to = vec!["email1@budshome.com", "email2@budshome.com"];
    let send_from = "email3@budshome.com";
    let passwd = "your_password";
    let subject = domain_valid.to_string() + "可注册";
    let body = domain_valid.to_string() + "可注册";
    let smtp_server = "smtp.budshome.com"; //"smtp.exmail.qq.com";

    let mut builder = EmailBuilder::new();
    for to in send_to {
        builder.add_to(to);
    }

    builder.add_from(send_from);
    builder.set_subject(subject);
    builder.set_body(body);
    let email = builder.build().unwrap();

    let mut emailer = SmtpTransport::simple_builder(smtp_server).unwrap()
        .smtp_utf8(true)
        .hello_name(ClientId::Domain("budshome.com".to_string()))
        .credentials(Credentials::new(send_from.to_string(), passwd.to_string()))
        .smtp_utf8(true)
        .authentication_mechanism(Mechanism::Plain)
        .connection_reuse(ConnectionReuseParameters::ReuseUnlimited).build();

    let result = emailer.send(&email);
    if !result.is_ok() {
        println!("邮件投递错误：{:#?}", result);
    }
    
    emailer.close();
}

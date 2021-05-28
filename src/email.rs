use dotenv::dotenv;

use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub async fn send_email(domain_available: &str) {
    dotenv().ok();

    let email_smtp = dotenv::var("email_smtp")
        .expect("Expected email_smtp to be set in env!");
    let email_from = dotenv::var("email_from")
        .expect("Expected email_from to be set in env!");
    let email_username = dotenv::var("email_username")
        .expect("Expected email_username to be set in env!");
    let email_password = dotenv::var("email_password")
        .expect("Expected email_password to be set in env!");
    let email_to =
        dotenv::var("email_to").expect("Expected email_to to be set in env!");

    let email = Message::builder()
        .from(email_from.parse().unwrap())
        .to(email_to.parse().unwrap())
        .subject(domain_available.to_string() + " 可注册")
        .body(domain_available.to_string())
        .unwrap();

    let creds = Credentials::new(email_username, email_password);

    // Open a remote connection to qq.com
    let mailer =
        SmtpTransport::relay(&email_smtp).unwrap().credentials(creds).build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}

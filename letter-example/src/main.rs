extern crate uuid;
extern crate lettre;
extern crate native_tls;

use std::env;
use lettre::{SendableEmail, EmailAddress, EmailTransport};
use lettre::smtp::{SmtpTransportBuilder, SUBMISSION_PORT};
use lettre::smtp::authentication::Credentials;
use lettre::smtp::client::net::ClientTlsParameters;

use native_tls::TlsConnector;

#[derive(Debug)]
struct CrashReport {
    to: Vector<EmailAddress>,
    from: EmailAddress,
    message_id: String,
    message: Vec<u8>,
}

fn main() {
    println!("Hello, world!");
}

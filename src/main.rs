use std::net::TcpListener;

use zero2prod::configuration::Configuration;
use zero2prod::startup;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = Configuration::get().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind radom port");

    startup::run(listener)?.await
}

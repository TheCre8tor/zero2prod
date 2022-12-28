//! main.rs

use zero2prod::configuration::get_configuration;
use zero2prod::startup::Application;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Telemetry Setup & Initialization ->
    //! init_subscriber should only be called once
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Environment Settings ->
    let configuration = get_configuration().expect("Failed to read configuration");

    // Server Initialization ->
    let application = Application::build(configuration).await?;
    application.run_server().await?;

    Ok(())
}

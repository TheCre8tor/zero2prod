//! main.rs

use sqlx::postgres::PgPoolOptions;
// use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Telemetry Setup & Initialization ->
    //! init_subscriber should only be called once
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Database configuration ->
    let configuration = get_configuration().expect("Failed to read configuration");
    // let connection_pool =
    //     PgPool::connect_lazy(&configuration.database.connection_string().expose_secret())
    //         .expect("Failed to connect to Postgres.");
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.db_with_name());

    // Port & Address setup ->
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    let listener = TcpListener::bind(address)?;

    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    startup::run(listener, connection_pool)?.await
}

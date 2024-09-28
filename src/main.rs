use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::Configuration;
use zero2prod::startup;
use zero2prod::telemetry::Telemetry;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = Configuration::get().expect("Failed to read configuration.");

    Telemetry::init_subscriber(config.application.name, "info".into(), std::io::stdout);

    let address = format!("127.0.0.1:{}", config.application.port);
    let listener = TcpListener::bind(address)?;

    let connection_pool = PgPool::connect(config.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");

    startup::run(listener, connection_pool)?.await?;

    Ok(())
}

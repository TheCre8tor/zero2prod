use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use wiremock::MockServer;
use zero2prod::configuration::{get_configuration, DatabaseSettings};
use zero2prod::startup::{get_connection_pool, Application};
use zero2prod::telemetry::{get_subscriber, init_subscriber};

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub enum VirtualDB {
    Enabled,
    Disabled,
}

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
    pub email_server: MockServer,
}

impl TestApp {
    pub async fn post_subscriptions(&self, body: String) -> reqwest::Response {
        reqwest::Client::new()
            .post(&format!("{}/subscriptions", &self.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub async fn spawn_app(enable_db: VirtualDB) -> TestApp {
    // The first time `initialize` is invoked the code in `TRACING` is executed.
    // All other invocations will instead skip execution.
    //! init_subscriber should only be called once
    Lazy::force(&TRACING);

    // Launch a mock server to stand in for Postmark's API
    let email_server = MockServer::start().await;

    // Database connection setup ->
    // Randomise configuration to ensure test isolation
    let configuration = {
        let mut configs = get_configuration().expect("Failed to read configuration.");
        // Use a different database for each test case
        configs.database.database_name = Uuid::new_v4().to_string();
        // Use the mock server as email API
        configs.email_client.base_url = email_server.uri();
        // Use a random OS port
        configs.application.port = 0;

        configs
    };

    // Create and migrate the database
    match enable_db {
        VirtualDB::Enabled => {
            configure_database(&configuration.database).await;
        }
        VirtualDB::Disabled => {}
    }

    // Launch the application as a background task
    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application");

    let address = format!("http://127.0.0.1:{}", application.port());
    let db_pool = get_connection_pool(&configuration.database);

    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(application.run_server());

    // We return the application address to the caller!
    TestApp {
        address,
        db_pool,
        email_server,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create new database for test ->
    let mut connection = PgConnection::connect_with(&config.db_without_name())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    // Migrate the database
    let connection_pool = PgPool::connect_with(config.db_with_name())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    // Return the connection pool to the caller ->
    connection_pool
}

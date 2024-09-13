//! tests/health_check.rs

use reqwest::Client;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::{
    configuration::{Configuration, DatabaseSettings},
    startup,
};

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

impl TestApp {
    async fn spawn_app() -> TestApp {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind radom port");
        let socket_address = listener.local_addr().unwrap();
        let port = socket_address.port();

        let address = format!("http://127.0.0.1:{}", port);

        let mut configuration = Configuration::get().expect("Failed to read configuration.");

        // We randomly create new database name for test purposes
        configuration.database.database_name = Uuid::new_v4().to_string();

        let connection_pool = TestApp::configure_database(&configuration.database).await;

        let server =
            startup::run(listener, connection_pool.clone()).expect("Failed to bind address");

        let _ = tokio::spawn(server);

        Self {
            address,
            db_pool: connection_pool,
        }
    }

    pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
        let maintenance_settings = DatabaseSettings {
            database_name: "postgres".to_string(),
            username: "postgres".to_string(),
            password: "password".to_string(),
            ..config.clone()
        };

        let mut connection = PgConnection::connect(&maintenance_settings.connection_string())
            .await
            .expect("Failed to connect to Postgres.");

        // Create database.
        let create_query = format!(r#"CREATE DATABASE "{}"; "#, config.database_name);

        connection
            .execute(create_query.as_str())
            .await
            .expect("Failed to create database.");

        // Migrate database.
        let connection_pool = PgPool::connect(&maintenance_settings.connection_string())
            .await
            .expect("Failed to connect to Postgres.");

        sqlx::migrate!("./migrations")
            .run(&connection_pool)
            .await
            .expect("Failed to migrate the database");

        connection_pool
    }
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = TestApp::spawn_app().await;
    let client = Client::new();

    // Act
    let response = client
        .get(format!("{}/health-check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = TestApp::spawn_app().await;
    let client = Client::new();

    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app = TestApp::spawn_app().await;
    let client = Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        )
    }
}

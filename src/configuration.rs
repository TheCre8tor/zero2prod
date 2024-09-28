//! src/configuration.rs

use config::{Config, ConfigError, File, FileFormat};
use redact::Secret;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub name: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        ))
    }
}

pub struct Configuration;

impl Configuration {
    pub fn get() -> Result<Settings, ConfigError> {
        // Initialise our configuration reader
        let settings = Config::builder()
            .add_source(File::new("configuration.yaml", FileFormat::Yaml))
            .build()?;

        settings.try_deserialize::<Settings>()
    }
}

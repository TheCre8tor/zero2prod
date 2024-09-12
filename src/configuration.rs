//! src/configuration.rs

use config::{Config, ConfigError, File, FileFormat};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
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

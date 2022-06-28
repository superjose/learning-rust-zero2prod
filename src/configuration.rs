//! src/configuration.rs
use config::Config;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialize our configuration reader:
    // ADd configuration values from a file named `configuration`.
    // It will look for any top-level file with an extension
    // that `config` knows how to parse: yaml, json, etc.

    let settings = Config::builder()
        .add_source(config::File::with_name("configuration"))
        .build()?;

    settings.try_deserialize::<Settings>()
}

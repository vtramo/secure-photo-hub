use std::path::Path;
use anyhow::Context;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct DatabaseConfig {
    pub username: String,
    pub password: String,
    pub port: String,
    pub host: String,
    pub name: String,
}

pub fn setup_database_config(application_properties_path: &str, vault_secrets_path: &str) -> anyhow::Result<DatabaseConfig> {
    let config = config::Config::builder()
        .add_source(config::Environment::with_prefix("DB"))
        .add_source(config::File::new(vault_secrets_path, config::FileFormat::Yaml))
        .add_source(config::File::new(application_properties_path, config::FileFormat::Yaml))
        .build()
        .context("bad database config")?;

    Ok(DatabaseConfig {
        username: config.get::<String>("database.username").unwrap(),
        password: config.get::<String>("database.password").unwrap(),
        port: config.get::<String>("database.port").unwrap(),
        host: config.get::<String>("database.host").unwrap(),
        name: config.get::<String>("database.name").unwrap(),
    })
}
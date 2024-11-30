use std::env;
use std::fs::read_to_string;
use std::path::Path;
use std::sync::LazyLock;
use actix_web::dev::Server;

use anyhow::Context;
use yaml_rust2::{Yaml, YamlLoader};

use s3::setup_aws_config;

pub use setup::{oidc::OidcConfig, redis::RedisConfig, database::DatabaseConfig};

use crate::setup;
use crate::setup::database::{setup_database_config};
use crate::setup::http::spawn_http_server;
use crate::setup::logging::init_logging;
use crate::setup::oidc::{setup_oidc_config};
use crate::setup::redis::{setup_redis_config};

mod database;
mod http;
mod oidc;
mod redis;
mod s3;
mod utils;
mod logging;

const CONFIG_LOCATION_ENV_VAR: &'static str = "CONFIG_LOCATION";
const VAULT_SECRETS_LOCATION_ENV_VAR: &'static str = "VAULT_SECRETS_LOCATION";
const SERVER_PORT_ENV_VAR: &'static str = "SERVER_PORT";
const SERVER_PORT_FIELD: &'static str = "server.port";
const APPLICATION_PROPERTIES: LazyLock<&Path> =
    LazyLock::new(|| Path::new("resources/application-properties.yaml"));
const VAULT_SECRETS: LazyLock<&Path> = LazyLock::new(|| Path::new("resources/vault-secrets.yaml"));

#[derive(Debug, Clone)]
pub struct Config {
    oidc_config: OidcConfig,
    redis_config: RedisConfig,
    database_config: DatabaseConfig,
    aws_config: aws_config::SdkConfig,
    server_port: u16,
}

impl Config {
    pub fn oidc_config(&self) -> &OidcConfig {
        &self.oidc_config
    }

    pub fn redis_config(&self) -> &RedisConfig {
        &self.redis_config
    }
}

pub async fn spawn_app() -> anyhow::Result<Server> {
    init_logging()?;
    let configuration = setup::setup().await?;
    let server = spawn_http_server(configuration).await?;
    Ok(server)
}

async fn setup() -> anyhow::Result<Config> {
    let application_properties_path = get_application_properties_path();
    let vault_secrets_path = get_vault_secrets_path();
    let application_properties =
        read_to_string(&application_properties_path).unwrap_or("x:|".to_string());
    let secrets = read_to_string(&vault_secrets_path).unwrap_or("x:|".to_string());

    let root_application_properties = YamlLoader::load_from_str(&application_properties)
        .context("Failed to parse YAML from application properties")?
        .get(0)
        .cloned()
        .context("Failed to parse YAML from application properties")?;

    let root_vault_secrets = YamlLoader::load_from_str(&secrets)
        .unwrap_or(vec![])
        .get(0)
        .cloned()
        .unwrap_or(Yaml::Null);

    let oidc_config = setup_oidc_config(&root_application_properties, &root_vault_secrets).await?;
    let redis_config = setup_redis_config(&root_application_properties)?;
    let aws_config = setup_aws_config(&root_vault_secrets).await?;
    let server_port = get_server_port(&root_application_properties);
    let database_config = setup_database_config(&application_properties_path, &vault_secrets_path)?;

    Ok(Config {
        oidc_config,
        redis_config,
        database_config,
        aws_config,
        server_port,
    })
}

fn get_server_port(application_properties: &Yaml) -> u16 {
    application_properties[SERVER_PORT_FIELD]
        .as_str()
        .map(|s| s.to_string())
        .or_else(|| env::var(SERVER_PORT_ENV_VAR).ok())
        .and_then(|s| s.parse().ok())
        .unwrap_or(8085)
}

fn get_application_properties_path() -> String {
    env::var(CONFIG_LOCATION_ENV_VAR)
        .unwrap_or(APPLICATION_PROPERTIES.to_str().unwrap().to_string())
}

fn get_vault_secrets_path() -> String {
    env::var(VAULT_SECRETS_LOCATION_ENV_VAR).unwrap_or(VAULT_SECRETS.to_str().unwrap().to_string())
}

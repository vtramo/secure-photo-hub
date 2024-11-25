use std::env;
use std::fs::read_to_string;
use std::path::Path;
use std::sync::LazyLock;

use anyhow::Context;
use yaml_rust2::{Yaml, YamlLoader};

pub use http::init_http_server;
pub use oidc::{OidcConfig, OidcWellKnownConfig, setup_oidc_config};

use crate::setup::redis::{RedisConfig, setup_redis_config};

use s3::setup_aws_config;

mod http;
mod oidc;
mod redis;
mod utils;
mod s3;

const CONFIG_LOCATION_ENV_VAR: &'static str = "CONFIG_LOCATION";
const VAULT_SECRETS_LOCATION_ENV_VAR: &'static str = "VAULT_SECRETS_LOCATION";
const APPLICATION_PROPERTIES: LazyLock<&Path> = LazyLock::new(|| Path::new("resources/application-properties.yaml"));
const VAULT_SECRETS: LazyLock<&Path> = LazyLock::new(|| Path::new("resources/vault-secrets.yaml"));

#[derive(Debug, Clone)]
pub struct Config {
    oidc_config: OidcConfig,
    redis_config: RedisConfig,
    aws_config: aws_config::SdkConfig,
}

impl Config {
    pub fn oidc_config(&self) -> &OidcConfig {
        &self.oidc_config
    }

    pub fn redis_config(&self) -> &RedisConfig {
        &self.redis_config
    }
}

pub async fn setup() -> anyhow::Result<Config> {
    let application_properties = read_to_string(get_application_properties_path()).unwrap_or("x:|".to_string());
    let secrets = read_to_string(get_vault_secrets_path()).unwrap_or("x:|".to_string());

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

    Ok(Config {
        oidc_config,
        redis_config,
        aws_config,
    })
}

fn get_application_properties_path() -> String {
    env::var(CONFIG_LOCATION_ENV_VAR)
        .unwrap_or(APPLICATION_PROPERTIES
            .to_str()
            .unwrap()
            .to_string())
}

fn get_vault_secrets_path() -> String {
    env::var(VAULT_SECRETS_LOCATION_ENV_VAR)
        .unwrap_or(VAULT_SECRETS
            .to_str()
            .unwrap()
            .to_string())
}
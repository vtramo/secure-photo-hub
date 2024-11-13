use std::fs::read_to_string;

use anyhow::Context;
use yaml_rust2::YamlLoader;

pub use http::init_http_server;
pub use oidc::{OidcConfig, OidcWellKnownConfig, setup_oidc_config};

use crate::setup::redis::{RedisConfig, setup_redis_config};

mod http;
mod oidc;
mod redis;
mod utils;

const APPLICATION_PROPERTIES: &str = "resources/application-properties.yaml";

#[derive(Debug, Clone)]
pub struct Config {
    oidc_config: OidcConfig,
    redis_config: RedisConfig,
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
    let application_properties = read_to_string(APPLICATION_PROPERTIES).unwrap_or("x:|".to_string());

    let root = YamlLoader::load_from_str(&application_properties)
        .context("Failed to parse YAML from application properties")?
        .get(0)
        .cloned()
        .context("Failed to parse YAML from application properties")?;

    let oidc_config = setup_oidc_config(&root).await?;
    let redis_config = setup_redis_config(&root)?;

    Ok(Config {
        oidc_config,
        redis_config,
    })
}
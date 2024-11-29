use std::env;

use anyhow::Context;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use yaml_rust2::Yaml;

use crate::setup::utils;

const REDIS_HOST_ENV_VAR: &str = "REDIS_HOST";
const REDIS_CONFIG_KEY: &str = "redis";
const REDIS_HOST_KEY: &str = "host";
const MISSING_REDIS_HOST_MSG: &str = "Missing or invalid 'host' field in redis configuration";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RedisConfig {
    #[serde(
        deserialize_with = "utils::deserialize_url",
        serialize_with = "utils::serialize_url"
    )]
    connection_string: Url,
}

impl RedisConfig {
    pub fn connection_string(&self) -> &Url {
        &self.connection_string
    }
}

pub fn setup_redis_config(root: &Yaml) -> anyhow::Result<RedisConfig> {
    let redis = &root[REDIS_CONFIG_KEY];

    let redis_host = extract_redis_host(redis)?;

    Ok(RedisConfig {
        connection_string: redis_host,
    })
}

fn extract_redis_host(root: &Yaml) -> anyhow::Result<Url> {
    env::var(REDIS_HOST_ENV_VAR)
        .context("Environment variable REDIS_HOST is not set or is empty")
        .and_then(|host| {
            Url::parse(&host).context("Failed to parse REDIS_HOST environment variable as URL")
        })
        .or_else(|_| {
            root[REDIS_HOST_KEY]
                .as_str()
                .context(MISSING_REDIS_HOST_MSG)
                .and_then(|host_str| {
                    Url::parse(host_str).context("Failed to parse 'redis.host' field as URL")
                })
        })
}

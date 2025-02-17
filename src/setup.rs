use std::env;
use std::fs::read_to_string;
use std::path::Path;
use std::sync::LazyLock;

use actix_web::dev::Server;
use anyhow::Context;
use url::Url;
use yaml_rust2::{Yaml, YamlLoader};

use s3::setup_aws_s3_config;
pub use setup::{
    database::DatabaseConfig,
    http::AlbumRoutesState,
    http::ImageRoutesState,
    http::PhotoRoutesState,
    oidc::OidcConfig,
    redis::RedisConfig,
    s3::AwsS3Config};

use crate::setup;
use crate::setup::database::setup_database_config;
use crate::setup::http::create_http_server;
use crate::setup::logging::init_logging;
use crate::setup::oidc::setup_oidc_config;
use crate::setup::redis::setup_redis_config;

mod database;
mod http;
mod oidc;
mod redis;
mod s3;
mod utils;
mod logging;

const CONFIG_LOCATION_ENV_VAR: &'static str = "CONFIG_LOCATION";
const SECRETS_LOCATION_ENV_VAR: &'static str = "SECRETS_LOCATION";
const SERVER_PORT_ENV_VAR: &'static str = "SERVER_PORT";
const SERVER_FIELD: &'static str = "server";
const PORT_FIELD: &'static str = "port";
const IMAGE_REFERENCE_ENDPOINT_URL: &'static str = "image-reference-endpoint-url";
const APPLICATION_PROPERTIES: LazyLock<&Path> =
    LazyLock::new(|| Path::new("resources/application-properties.yaml"));
const SECRETS: LazyLock<&Path> = LazyLock::new(|| Path::new("resources/application-secrets.yaml"));

#[derive(Debug, Clone)]
pub struct Config {
    oidc_config: OidcConfig,
    redis_config: RedisConfig,
    database_config: DatabaseConfig,
    aws_s3_config: AwsS3Config,
    server_port: u16,
    image_reference_endpoint_url: Url
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
    let server = create_http_server(configuration).await?;
    Ok(server)
}

async fn setup() -> anyhow::Result<Config> {
    let application_properties_path = get_application_properties_path();
    let secrets_path = get_secrets_path();
    let application_properties =
        read_to_string(&application_properties_path).unwrap_or("x:|".to_string());
    let secrets = read_to_string(&secrets_path).unwrap_or("x:|".to_string());

    let root_application_properties = YamlLoader::load_from_str(&application_properties)
        .context("Failed to parse YAML from application properties")?
        .get(0)
        .cloned()
        .context("Failed to parse YAML from application properties")?;

    let root_secrets = YamlLoader::load_from_str(&secrets)
        .unwrap_or(vec![])
        .get(0)
        .cloned()
        .unwrap_or(Yaml::Null);

    let oidc_config = setup_oidc_config(&root_application_properties, &root_secrets).await?;
    let redis_config = setup_redis_config(&root_application_properties)?;
    let aws_s3_config = setup_aws_s3_config(&root_secrets).await?;
    let server_port = get_server_port(&root_application_properties);
    let database_config = setup_database_config(&application_properties_path, &secrets_path)?;
    let image_reference_endpoint_url = extract_image_reference_endpoint_url(&root_application_properties)?;

    Ok(Config {
        oidc_config,
        redis_config,
        database_config,
        aws_s3_config,
        server_port,
        image_reference_endpoint_url,
    })
}

fn extract_image_reference_endpoint_url(application_properties: &Yaml) -> anyhow::Result<Url> {
    application_properties[IMAGE_REFERENCE_ENDPOINT_URL]
        .as_str()
        .context(format!("Missing {} property!", IMAGE_REFERENCE_ENDPOINT_URL))
        .and_then(|url| Url::parse(url).context(format!("Invalid URL format for {}", IMAGE_REFERENCE_ENDPOINT_URL)))
}

fn get_server_port(application_properties: &Yaml) -> u16 {
    application_properties[SERVER_FIELD][PORT_FIELD]
        .as_i64()
        .map(|s| s.to_string())
        .or_else(|| env::var(SERVER_PORT_ENV_VAR).ok())
        .and_then(|s| s.parse().ok())
        .unwrap_or(8085)
}

fn get_application_properties_path() -> String {
    env::var(CONFIG_LOCATION_ENV_VAR)
        .unwrap_or(APPLICATION_PROPERTIES.to_str().unwrap().to_string())
}

fn get_secrets_path() -> String {
    env::var(SECRETS_LOCATION_ENV_VAR).unwrap_or(SECRETS.to_str().unwrap().to_string())
}

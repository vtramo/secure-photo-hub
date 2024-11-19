use std::env;
use std::ops::Deref;

use anyhow::Context;
use jsonwebtoken::jwk::JwkSet;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use yaml_rust2::Yaml;

use crate::setup::utils;

const CLIENT_ID_FIELD: &'static str = "client-id";
const CLIENT_SECRET_FIELD: &'static str = "client-secret";
const AUTH_SERVER_URL_FIELD: &'static str = "auth-server.url";
const OIDC_FIELD: &'static str = "oidc";
const REDIRECT_URI_FIELD: &'static str = "redirect-uri";
const SCOPES_FIELD: &'static str = "scopes";

const OIDC_CLIENT_ID_ENV_VAR: &str = "OIDC_CLIENT_ID";
const OIDC_SCOPES_ENV_VAR: &str = "OIDC_SCOPES";
const OIDC_REDIRECT_URI_ENV_VAR: &str = "OIDC_REDIRECT_URI";
const OIDC_CLIENT_SECRET_ENV_VAR: &str = "OIDC_CLIENT_SECRET";
const OIDC_AUTH_SERVER_URL_ENV_VAR: &str = "OIDC_AUTH_SERVER_URL";

#[derive(Debug, Clone)]
pub struct OidcConfig {
    client_id: String,
    client_secret: String,
    auth_server_url: Url,
    redirect_uri: Url,
    scopes: Vec<String>,
    oidc_well_known_config: OidcWellKnownConfig,
    jwks: JwkSet,
}

impl OidcConfig {
    pub fn auth_server_url(&self) -> &Url {
        &self.auth_server_url
    }

    pub fn client_secret(&self) -> &str {
        &self.client_secret
    }

    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    pub fn jwks(&self) -> &JwkSet {
        &self.jwks
    }

    pub fn scopes(&self) -> &Vec<String> {
        &self.scopes
    }

    pub fn redirect_uri(&self) -> &Url {
        &self.redirect_uri
    }
}

impl Deref for OidcConfig {
    type Target = OidcWellKnownConfig;

    fn deref(&self) -> &Self::Target {
        &self.oidc_well_known_config
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OidcWellKnownConfig {
    issuer: String,
    #[serde(deserialize_with = "utils::deserialize_url", serialize_with = "utils::serialize_url")]
    authorization_endpoint: Url,
    #[serde(deserialize_with = "utils::deserialize_url", serialize_with = "utils::serialize_url")]
    token_endpoint: Url,
    #[serde(deserialize_with = "utils::deserialize_url", serialize_with = "utils::serialize_url")]
    introspection_endpoint: Url,
    #[serde(deserialize_with = "utils::deserialize_url", serialize_with = "utils::serialize_url")]
    userinfo_endpoint: Url,
    #[serde(deserialize_with = "utils::deserialize_url", serialize_with = "utils::serialize_url")]
    end_session_endpoint: Url,
    #[serde(deserialize_with = "utils::deserialize_url", serialize_with = "utils::serialize_url")]
    jwks_uri: Url,
}

impl OidcWellKnownConfig {
    pub fn issuer(&self) -> &str {
        &self.issuer
    }

    pub fn authorization_endpoint(&self) -> &Url {
        &self.authorization_endpoint
    }

    pub fn token_endpoint(&self) -> &Url {
        &self.token_endpoint
    }

    pub fn introspection_endpoint(&self) -> &Url {
        &self.introspection_endpoint
    }

    pub fn userinfo_endpoint(&self) -> &Url {
        &self.userinfo_endpoint
    }

    pub fn end_session_endpoint(&self) -> &Url {
        &self.end_session_endpoint
    }

    pub fn jwks_uri(&self) -> &Url {
        &self.jwks_uri
    }
}

pub async fn setup_oidc_config(root_application_properties: &Yaml, root_secrets: &Yaml) -> anyhow::Result<OidcConfig> {
    let oidc = &root_application_properties[OIDC_FIELD];

    let client_id = extract_client_id(oidc, root_secrets)?;
    let client_secret = extract_client_secret(oidc, root_secrets)?;
    let auth_server_url = extract_auth_server_url(oidc)?;
    let redirect_uri = extract_redirect_uri(oidc)?;
    let scopes = extract_scopes(oidc)?;

    let well_know_openid_config_url = build_well_known_openid_config_url(&auth_server_url)?;
    let oidc_well_known_config = fetch_well_known_openid_config(&well_know_openid_config_url).await?;
    let jwks = fetch_jwks(&oidc_well_known_config.jwks_uri).await?;

    Ok(OidcConfig {
        client_id,
        client_secret,
        auth_server_url,
        scopes,
        redirect_uri,
        oidc_well_known_config,
        jwks,
    })
}

async fn fetch_jwks(jwks_url: &Url) -> anyhow::Result<JwkSet> {
    let http_client = reqwest::Client::new();

    let response = http_client
        .get(jwks_url.clone())
        .send()
        .await
        .context("Error during the HTTP request for the JWK")?;

    let jwks = response
        .json::<JwkSet>()
        .await
        .context("Error parsing the JWK JSON")?;

    Ok(jwks)
}

fn extract_client_id(root_application_properties: &Yaml, root_secrets: &Yaml) -> anyhow::Result<String> {
    env::var(OIDC_CLIENT_ID_ENV_VAR)
        .context("Environment variable OIDC_CLIENT_ID is not set or is empty")
        .or_else(|_| {
            root_application_properties[CLIENT_ID_FIELD]
                .as_str()
                .context("Missing or invalid 'client-id' field in OIDC configuration and 'OIDC_CLIENT_ID' environment variable is not set.")
                .or_else(|_| {
                    root_secrets[CLIENT_ID_FIELD]
                        .as_str()
                        .context("Missing or invalid 'client-id' field in secrets file")
                }).map(|id| id.to_string())
        })
}

fn extract_scopes(root: &Yaml) -> anyhow::Result<Vec<String>> {
    env::var(OIDC_SCOPES_ENV_VAR)
        .context("Environment variable OIDC_SCOPES is not set or is empty")
        .map(|scopes| scopes.split_whitespace().map(str::to_string).collect())
        .or_else(|_| {
            root[SCOPES_FIELD]
                .as_str()
                .context("Missing or invalid 'scopes' field in OIDC configuration and 'OIDC_SCOPES' environment variable is not set.")
                .map(|scopes| scopes.split_whitespace().map(str::to_string).collect())
        })
}

fn extract_redirect_uri(root: &Yaml) -> anyhow::Result<Url> {
    env::var(OIDC_REDIRECT_URI_ENV_VAR)
        .context("Environment variable OIDC_REDIRECT_URI is not set or is empty")
        .and_then(|uri| Url::parse(&uri).context("Failed to parse OIDC_REDIRECT_URI environment variable as URL"))
        .or_else(|_| {
            root[REDIRECT_URI_FIELD]
                .as_str()
                .context("Missing or invalid 'redirect-uri' field in OIDC configuration and 'OIDC_REDIRECT_URI' environment variable is not set.")
                .and_then(|uri_str| Url::parse(uri_str).context("Failed to parse 'redirect-uri' field as URL"))
        })
}

fn extract_client_secret(root: &Yaml, root_secrets: &Yaml) -> anyhow::Result<String> {
    env::var(OIDC_CLIENT_SECRET_ENV_VAR)
        .context("Environment variable OIDC_CLIENT_SECRET is not set or is empty")
        .or_else(|_| {
            root[CLIENT_SECRET_FIELD]
                .as_str()
                .context("Missing or invalid 'client-secret' field in OIDC configuration and 'OIDC_CLIENT_SECRET' environment variable is not set.")
                .or_else(|_| {
                    root_secrets[CLIENT_SECRET_FIELD]
                        .as_str()
                        .context("Missing or invalid 'client-secret' field in secrets configuration")
                }).map(|id| id.to_string())
        })
}

fn extract_auth_server_url(root: &Yaml) -> anyhow::Result<Url> {
    env::var(OIDC_AUTH_SERVER_URL_ENV_VAR)
        .context("Environment variable OIDC_AUTH_SERVER_URL is not set or is empty")
        .and_then(|url| Url::parse(&url).context("Failed to parse OIDC_AUTH_SERVER_URL environment variable as URL"))
        .or_else(|_| {
            root[AUTH_SERVER_URL_FIELD]
                .as_str()
                .context("Missing or invalid 'auth-server.url' field in OIDC configuration and 'OIDC_AUTH_SERVER_URL' environment variable is not set.")
                .and_then(|url_str| Url::parse(url_str).context("Failed to parse 'auth-server.url' field as URL"))
        })
}


fn build_well_known_openid_config_url(auth_server_url: &Url) -> anyhow::Result<reqwest::Url> {
    let well_known_url = format!("{}/.well-known/openid-configuration", auth_server_url);
    reqwest::Url::parse(&well_known_url)
        .context("Failed to build well-known OpenID configuration URL from auth server URL")
}

async fn fetch_well_known_openid_config(well_know_openid_config_url: &reqwest::Url) -> anyhow::Result<OidcWellKnownConfig> {
    let client = reqwest::Client::new();

    let response = client
        .get(well_know_openid_config_url.clone())
        .send()
        .await
        .context("Failed to send request to OIDC well-known endpoint")?;

    let config = response
        .json::<OidcWellKnownConfig>()
        .await
        .context("Failed to parse OIDC well-known configuration JSON")?;

    Ok(config)
}
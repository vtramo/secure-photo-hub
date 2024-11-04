use std::ops::Deref;

use anyhow::Context;
use jsonwebtoken::jwk::JwkSet;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use yaml_rust2::{Yaml};

use crate::setup::utils;

const CLIENT_ID_FIELD: &'static str = "client-id";
const CLIENT_SECRET_FIELD: &'static str = "client-secret";
const AUTH_SERVER_URL_FIELD: &'static str = "auth-server.url";
const OIDC_FIELD: &'static str = "oidc";
const REDIRECT_URI_FIELD: &'static str = "redirect-uri";
const SCOPES_FIELD: &'static str = "scopes";

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

pub async fn setup_oidc_config(root: &Yaml) -> anyhow::Result<OidcConfig> {
    let oidc = &root[OIDC_FIELD];

    let client_id = extract_client_id(oidc)?;
    let client_secret = extract_client_secret(oidc)?;
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

fn extract_client_id(root: &Yaml) -> anyhow::Result<String> {
    Ok(root[CLIENT_ID_FIELD]
        .as_str()
        .context("Missing or invalid 'client-id' field in oidc configuration")?
        .to_string())
}

fn extract_scopes(root: &Yaml) -> anyhow::Result<Vec<String>> {
    Ok(root[SCOPES_FIELD]
        .as_str()
        .context("Missing or invalid 'scopes' field in oidc configuration")?
        .to_string()
        .split(" ")
        .map(str::to_string)
        .collect())
}

fn extract_redirect_uri(root: &Yaml) -> anyhow::Result<Url> {
    Ok(Url::parse(
        root[REDIRECT_URI_FIELD].as_str()
            .context("Missing or invalid 'redirect-uri' field in oidc configuration")?
    )?)
}

fn extract_client_secret(root: &Yaml) -> anyhow::Result<String> {
    Ok(root[CLIENT_SECRET_FIELD]
        .as_str()
        .context("Missing or invalid 'client-secret' field in oidc configuration")?
        .to_string())
}

fn extract_auth_server_url(root: &Yaml) -> anyhow::Result<reqwest::Url> {
    Ok(Url::parse(
        root[AUTH_SERVER_URL_FIELD].as_str()
            .context("Missing or invalid 'auth-server.url' field in oidc configuration")?
    )?)
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
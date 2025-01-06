use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use async_trait::async_trait;

pub use authorization_request::{
    OAuthAuthorizationRequestState, OAuthResponseType, OAuthSecureAuthorizationRequest,
};
pub use claims::IdTokenClaims;
pub use redirect_endpoint::oidc_redirect_endpoint;
pub use refresh_token_request::OAuthRefreshTokenRequest;
pub use token_validator::{
    authorization_check, validate_access_token, validate_id_token, OAuthValidatedTokens,
};
pub use user_info_request::{UserInfoEndpoint, UserInfoResponse};
pub use client_session::OAuthClientSession;

mod access_token_request;
mod authorization_request;
mod claims;
mod redirect_endpoint;
mod refresh_token_request;
mod token_validator;
mod user_info_request;
mod client_session;

pub const OAUTH_AUTHORIZATION_REQUEST_STATE_SESSION_KEY: &'static str =
    "oauth_authorization_request_state";
pub const OAUTH_SESSION_KEY: &'static str = "oauth_session";

#[derive(Deserialize, Serialize, Debug)]
pub struct OAuthSession {
    access_token_exp: u64,
    refresh_token_exp: u64,
    id_token_exp: u64,
    session_tokens: OAuthSessionTokens,
}

impl OAuthSession {
    pub fn new(
        access_token_exp: u64,
        refresh_token_exp: u64,
        id_token_exp: u64,
        oauth_session_tokens: OAuthSessionTokens,
    ) -> Self {
        Self {
            access_token_exp,
            refresh_token_exp,
            id_token_exp,
            session_tokens: oauth_session_tokens,
        }
    }

    pub fn session_tokens(&self) -> &OAuthSessionTokens {
        &self.session_tokens
    }

    pub fn is_access_token_expired(&self) -> bool {
        Self::now_as_secs() >= self.access_token_exp
    }

    pub fn is_refresh_token_expired(&self) -> bool {
        Self::now_as_secs() >= self.refresh_token_exp
    }

    pub fn is_id_token_expired(&self) -> bool {
        Self::now_as_secs() >= self.id_token_exp
    }
    fn now_as_secs() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("The current time should always be after UNIX_EPOCH")
            .as_secs()
    }
}

impl From<&OAuthValidatedTokens> for OAuthSession {
    fn from(validated_tokens: &OAuthValidatedTokens) -> Self {
        let session_tokens = OAuthSessionTokens::from(validated_tokens);
        Self::new(
            validated_tokens.access_token_exp(),
            validated_tokens.refresh_token_exp(),
            validated_tokens.id_token_exp(),
            session_tokens,
        )
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OAuthSessionTokens {
    access_token: String,
    refresh_token: String,
    id_token: String,
    nonce: Option<String>,
}

impl From<&OAuthValidatedTokens> for OAuthSessionTokens {
    fn from(validated_tokens: &OAuthValidatedTokens) -> Self {
        let id_token_claims = validated_tokens.id_token_claims();
        let nonce = id_token_claims.nonce().as_deref();
        Self::new(
            validated_tokens.access_token(),
            validated_tokens.refresh_token(),
            validated_tokens.id_token(),
            nonce,
        )
    }
}

impl OAuthSessionTokens {
    pub fn new(
        access_token: &str,
        refresh_token: &str,
        id_token: &str,
        nonce: Option<&str>,
    ) -> Self {
        Self {
            access_token: access_token.to_string(),
            refresh_token: refresh_token.to_string(),
            id_token: id_token.to_string(),
            nonce: nonce.map(|s| s.to_string()),
        }
    }

    pub fn access_token(&self) -> &str {
        &self.access_token
    }

    pub fn refresh_token(&self) -> &str {
        &self.refresh_token
    }

    pub fn id_token(&self) -> &str {
        &self.id_token
    }

    pub fn nonce(&self) -> &Option<String> {
        &self.nonce
    }
}

#[derive(Deserialize, Debug)]
pub struct OAuthAuthorizationResponse {
    access_token: String,
    refresh_token: String,
    id_token: String,
    expires_in: u64,
    refresh_expires_in: u64,
    #[serde(rename = "not-before-policy")]
    not_before_policy: u64,
    #[serde(default)]
    session_state: String,
    scope: String,
}

impl OAuthAuthorizationResponse {
    pub fn access_token(&self) -> &str {
        &self.access_token
    }

    pub fn refresh_token(&self) -> &str {
        &self.refresh_token
    }

    pub fn id_token(&self) -> &str {
        &self.id_token
    }

    pub fn expires_in(&self) -> u64 {
        self.expires_in
    }

    pub fn refresh_expires_in(&self) -> u64 {
        self.refresh_expires_in
    }

    pub fn not_before_policy(&self) -> u64 {
        self.not_before_policy
    }

    pub fn session_state(&self) -> &str {
        &self.session_state
    }

    pub fn scope(&self) -> &str {
        &self.scope
    }
}

#[async_trait()]
pub trait OAuthAccessTokenHolder {
    async fn get_access_token(&self) -> anyhow::Result<String>;
}
use anyhow::anyhow;
use jsonwebtoken::jwk::JwkSet;
use url::Url;

use crate::security::auth::oauth::access_token_request::OAuthAccessTokenRequest;
use crate::security::auth::oauth::{authorization_check, OAuthRefreshTokenRequest, OAuthSession, OAuthSessionTokens};
use crate::setup::OidcConfig;

pub struct OAuthClientSession {
    client_id: String,
    jwk_set: JwkSet,
    session: OAuthSession,
    refresh_token_request: OAuthRefreshTokenRequest
}

impl OAuthClientSession {
    pub async fn start_session_from_oidc_config(oidc_config: &OidcConfig) -> anyhow::Result<Self> {
        Self::start_session(
            oidc_config.client_id(),
            oidc_config.client_secret(),
            oidc_config.token_endpoint().clone(),
            oidc_config.jwks(),
            oidc_config.scopes()
        ).await
    }
    
    pub async fn start_session(
        client_id: &str,
        client_secret: &str,
        token_endpoint: Url,
        jwk_set: &JwkSet,
        scopes: &[String],
    ) -> anyhow::Result<Self> {
        let access_token_request = OAuthAccessTokenRequest::client_credentials(&token_endpoint, client_id, client_secret);
        let authorization_response = access_token_request.send_client_credentials_request(&scopes).await?;
        let session_tokens = OAuthSessionTokens::new(
            authorization_response.access_token(),
            authorization_response.refresh_token(),
            authorization_response.id_token(),
            None
        );
        
        let validated_tokens = authorization_check(&session_tokens, jwk_set, client_id)
            .await
            .ok_or(anyhow!("Authorization check failed!"))?;
        
        let oauth_session = OAuthSession::new(
            validated_tokens.access_token_exp(),
            validated_tokens.refresh_token_exp(),
            validated_tokens.id_token_exp(),
            session_tokens,
        );
        
        let refresh_token_request = OAuthRefreshTokenRequest::new(
            &token_endpoint, 
            client_id, 
            client_secret, 
            scopes, 
            validated_tokens.refresh_token()
        );
        
        Ok(Self {
            client_id: client_id.to_string(),
            jwk_set: jwk_set.clone(),
            session: oauth_session,
            refresh_token_request,
        })
    }
    
    pub async fn get_access_token(&mut self) -> anyhow::Result<String> {
        if self.session.is_access_token_expired() {
            return Ok(self.refresh_token().await?);
        }

        Ok(self.session.session_tokens().access_token().to_string())
    }
    
    async fn refresh_token(&mut self) -> anyhow::Result<String> {
        let authorization_response = self.refresh_token_request.send().await?;
        let session_tokens = OAuthSessionTokens::new(
            authorization_response.access_token(),
            authorization_response.refresh_token(),
            authorization_response.id_token(),
            None
        );
        let validated_tokens = authorization_check(&session_tokens, &self.jwk_set, &self.client_id)
            .await
            .ok_or(anyhow!("Authorization check failed!"))?;
        self.session = OAuthSession::from(&validated_tokens);
        return Ok(self.session.session_tokens().access_token().to_string())
    }
}
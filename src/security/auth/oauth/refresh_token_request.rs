use crate::security::auth::oauth::OAuthAuthorizationResponse;
use reqwest::Url;

pub struct OAuthRefreshTokenRequest {
    refresh_token_endpoint: Url,
    client_id: String,
    client_secret: String,
    scope: Vec<String>,
    refresh_token: String,
}

impl OAuthRefreshTokenRequest {
    const GRANT_TYPE: &'static str = "grant_type";
    const REFRESH_TOKEN: &'static str = "refresh_token";
    const CLIENT_ID: &'static str = "client_id";
    const CLIENT_SECRET: &'static str = "client_secret";
    const SCOPE: &'static str = "scope";

    pub fn new(
        refresh_token_endpoint: &Url,
        client_id: &str,
        client_secret: &str,
        scope: &[String],
        refresh_token: &str,
    ) -> Self {
        Self {
            refresh_token_endpoint: refresh_token_endpoint.clone(),
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            scope: scope.to_vec(),
            refresh_token: refresh_token.to_string(),
        }
    }

    pub async fn send(&self) -> anyhow::Result<OAuthAuthorizationResponse> {
        let client = reqwest::Client::new();
        let authorization_response = client
            .post(&self.refresh_token_endpoint.to_string())
            .form(&self.to_params())
            .send()
            .await?
            .json::<OAuthAuthorizationResponse>()
            .await?;

        Ok(authorization_response)
    }

    fn to_params(&self) -> Vec<(&'static str, String)> {
        vec![
            (Self::GRANT_TYPE, Self::REFRESH_TOKEN.to_string()),
            (Self::REFRESH_TOKEN, self.refresh_token.clone()),
            (Self::CLIENT_ID, self.client_id.clone()),
            (Self::CLIENT_SECRET, self.client_secret.clone()),
            (Self::SCOPE, self.scope.join(" ")),
        ]
    }
}

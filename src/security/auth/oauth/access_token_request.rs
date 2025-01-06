use reqwest::Url;

use crate::security::auth::oauth::OAuthAuthorizationResponse;

pub struct OAuthAccessTokenRequest {
    access_token_endpoint: Url,
    client_id: String,
    client_secret: String,
    redirect_uri: Option<Url>,
    code: Option<String>,
    code_verifier: Option<String>,
}

impl OAuthAccessTokenRequest {
    const GRANT_TYPE: &'static str = "grant_type";
    const AUTHORIZATION_CODE: &'static str = "authorization_code";
    const CLIENT_CREDENTIALS: &'static str = "client_credentials";
    const CODE: &'static str = "code";
    const CODE_VERIFIER: &'static str = "code_verifier";
    const CLIENT_ID: &'static str = "client_id";
    const CLIENT_SECRET: &'static str = "client_secret";
    const REDIRECT_URI: &'static str = "redirect_uri";
    const SCOPE: &'static str = "scope";

    pub fn authorization_code_request(
        access_token_endpoint: &Url,
        client_id: &str,
        client_secret: &str,
        redirect_uri: &Url,
        code: &str,
        code_verifier: &str,
    ) -> Self {
        Self {
            access_token_endpoint: access_token_endpoint.clone(),
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            redirect_uri: Some(redirect_uri.clone()),
            code: Some(code.to_string()),
            code_verifier: Some(code_verifier.to_string()),
        }
    }

    pub async fn send_authorization_code_request(&self) -> anyhow::Result<OAuthAuthorizationResponse> {
        let client = reqwest::Client::new();
        let authorization_response = client
            .post(&self.access_token_endpoint.to_string())
            .form(&self.to_authorization_code_params())
            .send()
            .await?
            .json::<OAuthAuthorizationResponse>()
            .await?;

        Ok(authorization_response)
    }

    fn to_authorization_code_params(&self) -> Vec<(&'static str, String)> {
        vec![
            (Self::GRANT_TYPE, Self::AUTHORIZATION_CODE.to_string()),
            (Self::CODE, self.code.clone().unwrap()),
            (Self::CODE_VERIFIER, self.code_verifier.clone().unwrap()),
            (Self::CLIENT_ID, self.client_id.clone()),
            (Self::CLIENT_SECRET, self.client_secret.clone()),
            (Self::REDIRECT_URI, self.redirect_uri.clone().unwrap().to_string()),
        ]
    }
    
    pub fn client_credentials(
        access_token_endpoint: &Url,
        client_id: &str,
        client_secret: &str,
    ) -> Self {
        Self {
            access_token_endpoint: access_token_endpoint.clone(),
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            redirect_uri: None,
            code: None,
            code_verifier: None,
        }
    }
    
    pub async fn send_client_credentials_request(&self, scopes: &[String]) -> anyhow::Result<OAuthAuthorizationResponse> {
        let client = reqwest::Client::new();
        let authorization_response = client
            .post(&self.access_token_endpoint.to_string())
            .form(&self.to_client_credentials_params(scopes))
            .send()
            .await?
            .json::<OAuthAuthorizationResponse>()
            .await?;

        Ok(authorization_response)
    }
    
    fn to_client_credentials_params(&self, scopes: &[String]) -> Vec<(&'static str, String)> {
        vec![
            (Self::GRANT_TYPE, Self::CLIENT_CREDENTIALS.to_string()),
            (Self::CLIENT_ID, self.client_id.clone()),
            (Self::CLIENT_SECRET, self.client_secret.clone()),
            (Self::SCOPE, scopes.join(" ").to_string())
        ]
    }
}
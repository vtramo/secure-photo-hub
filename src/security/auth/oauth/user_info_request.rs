use reqwest::Url;
use anyhow::Result;
use serde::Deserialize;

pub struct UserInfoEndpoint {
    endpoint_url: Url,
    access_token: String,
}

impl UserInfoEndpoint {
    pub fn new(endpoint_url: &Url, access_token: &str) -> Self {
        Self {
            endpoint_url: endpoint_url.clone(),
            access_token: access_token.to_string(),
        }
    }

    pub async fn fetch_user_info(&self) -> Result<UserInfoResponse> {
        let client = reqwest::Client::new();

        let response = client
            .get(self.endpoint_url.clone())
            .bearer_auth(&self.access_token)
            .send()
            .await?
            .error_for_status()?
            .json::<UserInfoResponse>()
            .await?;

        Ok(response)
    }
}

#[derive(Debug, Deserialize)]
pub struct UserInfoResponse {
    sub: String,
    email_verified: bool,
    name: String,
    preferred_username: String,
    given_name: String,
    family_name: String,
    email: String,
}

impl UserInfoResponse {
    pub fn sub(&self) -> &str {
        &self.sub
    }

    pub fn email_verified(&self) -> bool {
        self.email_verified
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn preferred_username(&self) -> &str {
        &self.preferred_username
    }

    pub fn given_name(&self) -> &str {
        &self.given_name
    }

    pub fn family_name(&self) -> &str {
        &self.family_name
    }

    pub fn email(&self) -> &str {
        &self.email
    }
}
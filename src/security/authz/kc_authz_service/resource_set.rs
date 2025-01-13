use reqwest::Url;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct AuthzResourceSetRequest {
    resource_set_endpoint: Url,
    access_token: String,
    uri: String,
    matching_uri: bool,
    max: u32,
}

impl AuthzResourceSetRequest {
    const MATCHING_URI: &'static str = "matchingUri";
    const URI: &'static str = "uri";
    const MAX: &'static str = "max";
    const DEEP: &'static str = "deep";

    pub fn new(
        resource_set_endpoint: &Url,
        access_token: &str,
        path: &str, 
        max: u32, 
        matching_uri: bool
    ) -> Self {
        Self {
            resource_set_endpoint: resource_set_endpoint.clone(),
            access_token: access_token.to_string(),
            uri: path.to_string(),
            matching_uri,
            max,
        }
    }

    pub async fn send(&self) -> anyhow::Result<Vec<Uuid>> {
        let client = reqwest::Client::new();
        
        let resource_set_response = client
            .get(&self.resource_set_endpoint.to_string())
            .query(&self.to_params()?)
            .bearer_auth(self.access_token.to_string())
            .send()
            .await?
            .json::<Vec<Uuid>>()
            .await?;
        
        Ok(resource_set_response)
    }
    
    fn to_params(&self) -> anyhow::Result<Vec<(&'static str, String)>> {
        Ok(vec![
            (Self::MATCHING_URI, self.matching_uri.to_string()),
            (Self::URI, self.uri.to_string()),
            (Self::DEEP, false.to_string()),
            (Self::MAX, self.max.to_string()),
        ])
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthzResourceSetQueryParams<'a> {
    uri: &'a str,
    matching_uri: bool,
    max: u32,
}

impl<'a> AuthzResourceSetQueryParams<'a> {
    pub fn new(uri: &'a str, matching_uri: bool, max: u32) -> Self {
        Self {
            uri,
            matching_uri,
            max,
        }
    }
}
use reqwest::Url;
use serde::{Deserialize, Serialize};
use url::form_urlencoded;
use uuid::Uuid;

pub struct AuthzResourceSetRequest {
    resource_set_endpoint: Url,
    uri: String,
    matching_uri: bool,
    max: u32,
}

impl AuthzResourceSetRequest {
    const MATCHING_URI: &'static str = "matchingUri";
    const URI: &'static str = "uri";
    const MAX: &'static str = "max";
    const DEEP: &'static str = "deep";

    pub fn new(resource_set_endpoint: &Url, path: &str, max: u32, matching_uri: bool) -> Self {
        Self {
            resource_set_endpoint: resource_set_endpoint.clone(),
            uri: path.to_string(),
            matching_uri,
            max,
        }
    }

    pub async fn send(&self) -> anyhow::Result<Vec<Uuid>> {
        let client = reqwest::Client::new();
        
        let resource_set_response = dbg!(client
            .get(&self.resource_set_endpoint.to_string())
            .query(&self.to_params()?)
            .send()
            .await?
            .text()
            .await?);
        

        Ok(vec![])
    }
    
    fn to_params(&self) -> anyhow::Result<Vec<(&'static str, String)>> {
        Ok(vec![
            (Self::MATCHING_URI, self.matching_uri.to_string()),
            (Self::URI, form_urlencoded::byte_serialize(self.uri.as_bytes()).collect()),
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
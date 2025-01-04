use std::fmt;
use std::fmt::{Display, Formatter};

use base64::Engine;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;
use crate::security::authz::kc_authz_service::AuthorizationScope;

#[derive(Debug)]
pub struct AuthzPermissionRequest<T>
    where
        T: Serialize
{
    access_token_endpoint: Url,
    resource_id: Uuid,
    subject_token: String,
    client_id: String,
    client_secret: String,
    claims: T,
    scopes: Vec<AuthorizationScope>,
}


impl<T> AuthzPermissionRequest<T>
    where
        T: Serialize {

    const GRANT_TYPE: &'static str = "grant_type";
    const PERMISSION: &'static str = "permission";
    const AUDIENCE: &'static str = "audience";
    const SUBJECT_TOKEN: &'static str = "subject_token";
    const CLAIM_TOKEN: &'static str = "claim_token";
    const CLAIM_TOKEN_FORMAT: &'static str = "claim_token_format";
    const RESPONSE_MODE: &'static str = "response_mode";
    const CLIENT_ID: &'static str = "client_id";
    const CLIENT_SECRET: &'static str = "client_secret";
    const SCOPE: &'static str = "scope";

    const GRANT_TYPE_UMA_TICKET: &'static str = "urn:ietf:params:oauth:grant-type:uma-ticket";
    const CLAIM_TOKEN_FORMAT_VALUE: &'static str = "urn:ietf:params:oauth:token-type:jwt";

    pub fn new(
        access_token_endpoint: &Url,
        resource_id: &Uuid,
        subject_token: &str,
        client_id: &str,
        client_secret: &str,
        claims: T,
        scopes: Vec<AuthorizationScope>
    ) -> Self {
        Self {
            access_token_endpoint: access_token_endpoint.clone(),
            resource_id: resource_id.clone(),
            subject_token: subject_token.to_string(),
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            claims,
            scopes
        }
    }

    pub async fn decision_response_mode_send(&self) -> anyhow::Result<bool> {
        let client = reqwest::Client::new();
        let decision = client.post(&self.access_token_endpoint.to_string())
            .form(&self.to_params(ResponseMode::Decision)?)
            .send()
            .await?
            .json::<Decision>()
            .await?;

        Ok(decision.result)
    }
    
    pub async fn permissions_response_mode_send(&self) -> anyhow::Result<Permissions> {
        let client = reqwest::Client::new();
        Ok(client.post(&self.access_token_endpoint.to_string())
            .form(&self.to_params(ResponseMode::Decision)?)
            .send()
            .await?
            .json::<Permissions>()
            .await?)
    }

    fn to_params(&self, response_mode: ResponseMode) -> anyhow::Result<Vec<(&'static str, String)>> {
        let base64_claim_token = base64::prelude::BASE64_STANDARD_NO_PAD.encode(serde_json::to_string(&self.claims)?);
        let client_id = self.client_id.to_string();
        Ok(vec![(Self::GRANT_TYPE, Self::GRANT_TYPE_UMA_TICKET.to_string()),
                (Self::PERMISSION, self.resource_id.to_string()),
                (Self::AUDIENCE, client_id.clone()),
                (Self::SUBJECT_TOKEN, self.subject_token.to_string()),
                (Self::CLAIM_TOKEN, base64_claim_token),
                (Self::CLAIM_TOKEN_FORMAT, Self::CLAIM_TOKEN_FORMAT_VALUE.to_string()),
                (Self::RESPONSE_MODE, response_mode.to_string()),
                (Self::CLIENT_ID, client_id),
                (Self::CLIENT_SECRET, self.client_secret.to_string()),
                (Self::SCOPE, self.scopes.iter().map(|scope| scope.to_string()).collect::<Vec<String>>().join(", "))])
    }
}

#[derive(Copy, Clone, Debug)]
enum ResponseMode {
    Decision,
    Permissions,
}

impl Display for ResponseMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ResponseMode::Decision => f.write_str("decision"),
            ResponseMode::Permissions => f.write_str("permissions"),
        }
    }
}

#[derive(Deserialize)]
struct Decision {
    result: bool
}

#[derive(Deserialize, Clone, Debug)]
pub struct Permissions {
    scopes: Vec<AuthorizationScope>,
    rsid: Uuid,
    rsname: String,
}
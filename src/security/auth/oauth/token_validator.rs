use jsonwebtoken::TokenData;
use serde_json::Value;
use crate::security::auth::oauth::{OAuthAuthorizationResponse, OAuthSessionTokens};
use crate::security::jwks::{IdTokenClaims, TokenValidationError, validate_access_token, validate_id_token};

#[derive(Debug)]
pub struct OAuthValidatedTokens {
    tokens_refreshed: bool,
    access_token: String,
    refresh_token: String,
    id_token: String,
    access_token_claims: TokenData<Value>,
    id_token_claims: IdTokenClaims,
}

impl OAuthValidatedTokens {
    pub fn is_tokens_refreshed(&self) -> bool {
        self.tokens_refreshed
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

    pub fn access_token_claims(&self) -> &TokenData<Value> {
        &self.access_token_claims
    }

    pub fn id_token_claims(&self) -> &IdTokenClaims {
        &self.id_token_claims
    }

    pub fn access_token_exp(&self) -> u64 {
        self.access_token_claims.claims.get("exp").unwrap().as_u64().unwrap()
    }

    pub fn refresh_token_exp(&self) -> u64 {
        self.access_token_claims.claims.get("exp").unwrap().as_u64().unwrap() // TODO
    }

    pub fn id_token_exp(&self) -> u64 {
        self.id_token_claims.exp()
    }
}

pub async fn authorization_check(session_token: &OAuthSessionTokens) -> Option<OAuthValidatedTokens> {
    let access_token = session_token.access_token.clone();
    let refresh_token = session_token.refresh_token.clone();
    let id_token = session_token.id_token.clone();
    let nonce = session_token.nonce.clone();

    match validate_tokens(&access_token, &id_token, nonce).await {
        Ok((access_token_claims, id_token_claims)) => Some(OAuthValidatedTokens {
            tokens_refreshed: false,
            access_token,
            refresh_token,
            id_token,
            access_token_claims,
            id_token_claims,
        }),

        Err(TokenValidationError::ExpiredSignature) => {
            log::warn!("Expired tokens");

            refresh_token_endpoint(&refresh_token).await
                .and_then(|(new_access_token, new_refresh_token, new_id_token, new_access_token_claims, new_id_token_claims)| {
                    Some(OAuthValidatedTokens {
                        tokens_refreshed: true,
                        access_token: new_access_token,
                        refresh_token: new_refresh_token,
                        id_token: new_id_token,
                        access_token_claims: new_access_token_claims,
                        id_token_claims: new_id_token_claims,
                    })
                })
        },

        _ => None,
    }
}

async fn validate_tokens(
    access_token: &str,
    id_token: &str,
    nonce: Option<String>
) -> Result<(TokenData<Value>, IdTokenClaims), TokenValidationError> {
    let id_token_claims = validate_id_token(&id_token, nonce).await?;
    let access_token_claims = validate_access_token(&access_token).await?;
    Ok((access_token_claims, id_token_claims))
}

async fn refresh_token_endpoint(refresh_token: &str) -> Option<(String, String, String, TokenData<Value>, IdTokenClaims)> {
    let params = [
        ("grant_type", "refresh_token"),
        ("refresh_token", refresh_token),
        ("client_id", "fast-photo-hub-rest-api"),
        ("client_secret", "ERMXusbPy62B1JiEGwT7bKMcal8mrwId"),
        ("scope", "openid profile email offline_access"),
    ];

    let client = reqwest::Client::new();
    let authorization_response = client
        .post("http://localhost:8080/realms/fast-photo-hub/protocol/openid-connect/token")
        .form(&params)
        .send()
        .await
        .ok()?
        .json::<OAuthAuthorizationResponse>()
        .await
        .ok()?;

    validate_tokens(&authorization_response.access_token, &authorization_response.id_token, None).await
        .map(|(access_token_claims, id_token_claims)|
            (authorization_response.access_token,
             authorization_response.refresh_token,
             authorization_response.id_token,
             access_token_claims,
             id_token_claims)).ok()
}
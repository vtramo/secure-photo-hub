use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::jwk::JwkSet;
use jsonwebtoken::{decode_header, Algorithm, DecodingKey, TokenData, Validation};
use serde_json::Value;

use crate::security::auth::oauth::claims::IdTokenClaims;
use crate::security::auth::oauth::OAuthSessionTokens;

#[derive(Debug)]
pub struct OAuthValidatedTokens {
    access_token: String,
    refresh_token: String,
    id_token: String,
    access_token_claims: TokenData<Value>,
    id_token_claims: IdTokenClaims,
}

impl OAuthValidatedTokens {
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
        self.access_token_claims
            .claims
            .get("exp")
            .unwrap()
            .as_u64()
            .unwrap()
    }

    pub fn refresh_token_exp(&self) -> u64 {
        self.access_token_claims
            .claims
            .get("exp")
            .unwrap()
            .as_u64()
            .unwrap() // TODO
    }

    pub fn id_token_exp(&self) -> u64 {
        self.id_token_claims.exp()
    }
}

pub enum TokenValidationError {
    ExpiredSignature,
    Unknown,
}

pub async fn authorization_check(
    session_token: &OAuthSessionTokens,
    jwks: &JwkSet,
    client_id: &str,
) -> Option<OAuthValidatedTokens> {
    log::info!("Authorization Check");

    let access_token = session_token.access_token.clone();
    let refresh_token = session_token.refresh_token.clone();
    let id_token = session_token.id_token.clone();
    let nonce = session_token.nonce.clone();

    match validate_tokens(&access_token, &id_token, nonce, jwks, client_id).await {
        Ok((access_token_claims, id_token_claims)) => Some(OAuthValidatedTokens {
            access_token,
            refresh_token,
            id_token,
            access_token_claims,
            id_token_claims,
        }),

        Err(TokenValidationError::ExpiredSignature) => {
            log::warn!("Expired tokens");
            None
        }

        _ => None,
    }
}

async fn validate_tokens(
    access_token: &str,
    id_token: &str,
    nonce: Option<String>,
    jwks: &JwkSet,
    client_id: &str,
) -> Result<(TokenData<Value>, IdTokenClaims), TokenValidationError> {
    let id_token_claims = validate_id_token(&id_token, nonce, jwks, client_id).await?;
    let access_token_claims = validate_access_token(&access_token, jwks).await?;
    Ok((access_token_claims, id_token_claims))
}

pub async fn validate_access_token(
    access_token: &str,
    jwks: &JwkSet,
) -> Result<TokenData<Value>, TokenValidationError> {
    let (decoding_key, alg) = extract_decoding_key(access_token, jwks)?;
    let mut validation = Validation::new(alg);
    validation.set_audience(&["account"]);

    match jsonwebtoken::decode::<Value>(access_token, &decoding_key, &validation) {
        Ok(token_data) => Ok(token_data),
        Err(err) => {
            log::error!("Error validating access token: {:?}", err);
            match err.kind() {
                ErrorKind::ExpiredSignature => Err(TokenValidationError::ExpiredSignature),
                _ => Err(TokenValidationError::Unknown),
            }
        }
    }
}

pub async fn validate_id_token(
    id_token: &str,
    nonce: Option<String>,
    jwks: &JwkSet,
    client_id: &str,
) -> Result<IdTokenClaims, TokenValidationError> {
    let (decoding_key, alg) = extract_decoding_key(id_token, jwks)?;
    let mut validation = Validation::new(alg);
    validation.set_audience(&[client_id]);

    match jsonwebtoken::decode::<IdTokenClaims>(id_token, &decoding_key, &validation) {
        Ok(token_data) => {
            let id_token_claims = token_data.claims;

            if let Some(id_token_nonce) = id_token_claims.nonce().clone() {
                if let Some(nonce) = nonce {
                    if id_token_nonce != nonce {
                        log::warn!(
                            "Nonce mismatch: expected {}, found {}",
                            nonce,
                            id_token_nonce
                        );
                        return Err(TokenValidationError::Unknown);
                    }
                }
            }

            Ok(id_token_claims)
        }

        Err(err) => match err.kind() {
            ErrorKind::ExpiredSignature => {
                log::warn!("ID token has expired: {}", err);
                Err(TokenValidationError::ExpiredSignature)
            }
            _ => {
                log::warn!("ID token validation failed: {}", err);
                Err(TokenValidationError::Unknown)
            }
        },
    }
}

fn extract_decoding_key(
    token: &str,
    jwks: &JwkSet,
) -> Result<(DecodingKey, Algorithm), TokenValidationError> {
    let header = decode_header(token).map_err(|err| {
        log::warn!("Failed to decode token header: {}", err);
        TokenValidationError::Unknown
    })?;

    let kid = header.kid.ok_or_else(|| {
        log::warn!("No 'kid' found in the token header.");
        TokenValidationError::Unknown
    })?;

    let jwk = jwks.find(&kid).ok_or_else(|| {
        log::warn!("No JWK found for the 'kid': {}", kid);
        TokenValidationError::Unknown
    })?;

    let decoding_key = DecodingKey::from_jwk(jwk).map_err(|err| {
        log::warn!("Failed to create DecodingKey from JWK: {}", err);
        TokenValidationError::Unknown
    })?;

    Ok((decoding_key, header.alg))
}

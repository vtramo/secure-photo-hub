use std::future::Future;
use std::pin::Pin;
use actix_session::SessionExt;
use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use serde::{Deserialize, Serialize};
use crate::security::auth::USER_SESSION_KEY;
use crate::security::jwks::IdTokenClaims;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: String,
    username: String,
    given_name: String,
    family_name: String,
    full_name: String,
    email: String,
    email_verified: bool,
}

impl User {
    pub fn full_name(&self) -> &str {
        &self.full_name
    }
}

impl From<&IdTokenClaims> for User {
    fn from(id_token_claims: &IdTokenClaims) -> Self {
        Self {
            id: id_token_claims.sub().to_string(),
            username: id_token_claims.preferred_username().to_string(),
            given_name: id_token_claims.given_name().to_string(),
            family_name: id_token_claims.family_name().to_string(),
            full_name: id_token_claims.name().to_string(),
            email: id_token_claims.email().to_string(),
            email_verified: id_token_claims.email_verified(),
        }
    }
}

impl FromRequest for User {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let session = req.get_session();

        Box::pin(async move {
            match session
                .get::<User>(USER_SESSION_KEY)
                .unwrap_or(None)
            {
                Some(user) => { Ok(user) },
                None => {
                    Err(ErrorUnauthorized("Unauthorized: No session tokens found"))
                }
            }
        })
    }
}
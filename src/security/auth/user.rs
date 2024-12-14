use std::future::Future;
use std::pin::Pin;

use crate::security::auth::middleware::AuthenticationMethod;
use actix_session::SessionExt;
use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use actix_web::{FromRequest, HttpMessage, HttpRequest};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::security::auth::oauth::{IdTokenClaims, UserInfoResponse};
use crate::security::auth::USER_SESSION_KEY;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    id: Uuid,
    username: String,
    given_name: String,
    family_name: String,
    full_name: String,
    email: String,
    email_verified: bool,
}

impl AuthenticatedUser {
    pub fn new(
        id: &Uuid, 
        username: &str, 
        given_name: &str, 
        family_name: &str, 
        full_name: &str, 
        email: &str, 
        email_verified: bool
    ) -> Self {
        Self { 
            id: id.clone(), 
            username: username.to_string(), 
            given_name: given_name.to_string(), 
            family_name: family_name.to_string(), 
            full_name: full_name.to_string(), 
            email: email.to_string(), 
            email_verified 
        }
    }
    pub fn id(&self) -> &Uuid {
        &self.id
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn given_name(&self) -> &str {
        &self.given_name
    }
    pub fn family_name(&self) -> &str {
        &self.family_name
    }
    pub fn full_name(&self) -> &str {
        &self.full_name
    }
    pub fn email(&self) -> &str {
        &self.email
    }
    pub fn email_verified(&self) -> bool {
        self.email_verified
    }
}

impl From<&IdTokenClaims> for AuthenticatedUser {
    fn from(id_token_claims: &IdTokenClaims) -> Self {
        Self {
            id: Uuid::parse_str(&id_token_claims.sub().to_string()).expect(""),
            username: id_token_claims.preferred_username().to_string(),
            given_name: id_token_claims.given_name().to_string(),
            family_name: id_token_claims.family_name().to_string(),
            full_name: id_token_claims.name().to_string(),
            email: id_token_claims.email().to_string(),
            email_verified: id_token_claims.email_verified(),
        }
    }
}

impl From<UserInfoResponse> for AuthenticatedUser {
    fn from(value: UserInfoResponse) -> Self {
        Self {
            id: Uuid::parse_str(&value.sub().to_string()).expect(""),
            username: value.preferred_username().to_string(),
            given_name: value.given_name().to_string(),
            family_name: value.family_name().to_string(),
            full_name: value.name().to_string(),
            email: value.email().to_string(),
            email_verified: value.email_verified(),
        }
    }
}

impl FromRequest for AuthenticatedUser {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let authenticated_user = get_authenticated_user(&req);
        Box::pin(async move {
            authenticated_user.ok_or(ErrorUnauthorized("Unauthorized: No session tokens found"))
        })
    }
}

fn get_authenticated_user(req: &HttpRequest) -> Option<AuthenticatedUser> {
    let session = req.get_session();
    let mut extensions = req.extensions_mut();
    let authentication_method = extensions.get::<AuthenticationMethod>()?;
    match authentication_method {
        AuthenticationMethod::OAuthCodeFlow => session.get::<AuthenticatedUser>(USER_SESSION_KEY).ok()?,
        AuthenticationMethod::Bearer => extensions.remove::<AuthenticatedUser>(),
    }
}

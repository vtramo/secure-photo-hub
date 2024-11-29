use std::ops::Deref;

use actix_session::Session;
use actix_web::web::Query;
use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::security::auth::oauth::access_token_request::OAuthAccessTokenRequest;
use crate::security::auth::oauth::{
    authorization_check, OAuthAuthorizationRequestState, OAuthSession, OAuthSessionTokens,
    OAUTH_AUTHORIZATION_REQUEST_STATE_SESSION_KEY, OAUTH_SESSION_KEY,
};
use crate::security::auth::user::User;
use crate::security::auth::USER_SESSION_KEY;
use crate::setup::Config;

#[derive(Debug, Deserialize)]
pub struct OAuthAuthorizationCode {
    code: String,
}

impl Deref for OAuthAuthorizationCode {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.code
    }
}

#[derive(Debug, Deserialize)]
pub struct OAuthState {
    state: String,
}

impl Deref for OAuthState {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

pub async fn oidc_redirect_endpoint(
    mut session: Session,
    code: Query<OAuthAuthorizationCode>,
    state: Query<OAuthState>,
    config: web::Data<Config>,
) -> HttpResponse {
    let oidc_config = config.oidc_config();

    let (nonce, code_verifier) = match restore_authorization_request_state(&mut session, &state.0) {
        None => return redirect_to_home(),
        Some((nonce, code_verifier)) => (nonce, code_verifier),
    };

    let access_token_request = OAuthAccessTokenRequest::authorization_code_request(
        oidc_config.token_endpoint(),
        oidc_config.client_id(),
        oidc_config.client_secret(),
        oidc_config.redirect_uri(),
        &code.0,
        &code_verifier,
    );

    if let Ok(authorization_response) = access_token_request.send().await {
        let session_tokens = OAuthSessionTokens::new(
            authorization_response.access_token(),
            authorization_response.refresh_token(),
            authorization_response.id_token(),
            Some(&nonce),
        );

        match authorization_check(&session_tokens, oidc_config.jwks(), oidc_config.client_id())
            .await
        {
            None => {}

            Some(validated_tokens) => {
                let oauth_session = OAuthSession::new(
                    validated_tokens.access_token_exp(),
                    validated_tokens.refresh_token_exp(),
                    validated_tokens.id_token_exp(),
                    session_tokens,
                );

                if let Err(e) = session.insert(OAUTH_SESSION_KEY, oauth_session) {
                    log::error!("Failed to insert OAuth session: {:?}", e);
                    return HttpResponse::InternalServerError().finish();
                }

                if let Err(e) = session.insert(
                    USER_SESSION_KEY,
                    User::from(validated_tokens.id_token_claims()),
                ) {
                    log::error!("Failed to insert user session: {:?}", e);
                    return HttpResponse::InternalServerError().finish();
                }
            }
        };
    }

    session.remove(OAUTH_AUTHORIZATION_REQUEST_STATE_SESSION_KEY);
    redirect_to_home()
}

fn restore_authorization_request_state(session: &Session, state: &str) -> Option<(String, String)> {
    match session
        .get::<OAuthAuthorizationRequestState>(OAUTH_AUTHORIZATION_REQUEST_STATE_SESSION_KEY)
        .ok()
    {
        Some(Some(authorization_request_state)) if authorization_request_state.state() == state => {
            Some((
                authorization_request_state.nonce().to_string(),
                authorization_request_state.code_verifier().to_string(),
            ))
        }

        _ => None,
    }
}

fn redirect_to_home() -> HttpResponse {
    HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish()
}

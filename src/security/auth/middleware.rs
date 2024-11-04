use actix_session::SessionExt;
use actix_web::{Error, HttpResponse, web};
use actix_web::body::{BoxBody, EitherBody, MessageBody};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::Next;
use reqwest::Url;

use crate::security::auth::oauth::{authorization_check, OAUTH_AUTHORIZATION_REQUEST_STATE_SESSION_KEY, OAUTH_SESSION_KEY, OAuthSecureAuthorizationRequest, OAuthSession};
use crate::security::auth::user::User;
use crate::security::auth::USER_SESSION_KEY;
use crate::setup::Config;

pub async fn authentication_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<EitherBody<BoxBody, impl MessageBody>>, Error> {
    let config = req.app_data::<web::Data<Config>>().expect("Failed to retrieve configuration from app data");
    let oidc_config = config.oidc_config();
    let oidc_redirect_uri = oidc_config.redirect_uri();

    let session = req.get_session();
    match session.get::<OAuthSession>(OAUTH_SESSION_KEY).unwrap_or(None) {
        None if req.path() != oidc_redirect_uri.as_str() => {
            let authorization_request = OAuthSecureAuthorizationRequest::code_flow(
                oidc_config.authorization_endpoint(),
                oidc_config.client_id(),
                oidc_config.scopes(),
                oidc_redirect_uri
            );

            session.insert(OAUTH_AUTHORIZATION_REQUEST_STATE_SESSION_KEY, authorization_request.state())?;

            let response = HttpResponse::Found()
                .append_header((http::header::LOCATION.to_string(), Url::from(&authorization_request).to_string()))
                .finish()
                .map_into_boxed_body();

            Ok(req.into_response(response).map_into_left_body())
        },

        Some(oauth_session) => {
            if oauth_session.is_access_token_expired() {
                if let Some(validated_tokens) = authorization_check(oauth_session.session_tokens()).await {
                    if validated_tokens.is_tokens_refreshed() {
                        session.insert(OAUTH_SESSION_KEY, OAuthSession::from(&validated_tokens))?;
                        session.insert(USER_SESSION_KEY, User::from(validated_tokens.id_token_claims()))?;
                    }
                }
            }

            Ok(next.call(req).await?.map_into_right_body())
        }

        _ => Ok(next.call(req).await?.map_into_right_body())
    }
}
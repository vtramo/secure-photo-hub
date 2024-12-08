use crate::routes;
use actix_session::SessionExt;
use actix_web::body::{BoxBody, EitherBody, MessageBody};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::Next;
use actix_web::{web, Error, HttpMessage, HttpResponse};
use reqwest::Url;

use crate::security::auth::oauth::{
    authorization_check, validate_access_token, OAuthRefreshTokenRequest,
    OAuthSecureAuthorizationRequest, OAuthSession, OAuthSessionTokens, UserInfoEndpoint,
    OAUTH_AUTHORIZATION_REQUEST_STATE_SESSION_KEY, OAUTH_SESSION_KEY,
};
use crate::security::auth::user::AuthenticatedUser;
use crate::security::auth::USER_SESSION_KEY;
use crate::setup::{Config, OidcConfig};

pub enum AuthenticationMethod {
    Bearer,
    OAuthCodeFlow,
}

pub async fn authentication_middleware_bearer_token(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<EitherBody<BoxBody, impl MessageBody>>, Error> {
    if is_health_check(&req) {
        return Ok(next.call(req).await?.map_into_right_body());
    }

    let oidc_config = get_oidc_config(&req)?;

    if let Some(ref access_token) = extract_bearer_token(&req) {
        match validate_access_token(&access_token, oidc_config.jwks()).await {
            Ok(_) => {
                let user_info_endpoint =
                    UserInfoEndpoint::new(oidc_config.userinfo_endpoint(), access_token);
                match user_info_endpoint.fetch_user_info().await {
                    Ok(user_info_response) => {
                        let user = AuthenticatedUser::from(user_info_response);
                        req.get_session().clear();
                        req.extensions_mut().insert(user);
                        req.extensions_mut().insert(AuthenticationMethod::Bearer);

                        Ok(next.call(req).await?.map_into_right_body())
                    }
                    Err(_) => {
                        log::error!("Failed to fetch user info from the UserInfo endpoint");
                        Ok(req.into_response(unauthorized()).map_into_left_body())
                    }
                }
            }
            Err(_) => Ok(req.into_response(unauthorized()).map_into_left_body()),
        }
    } else {
        Ok(next.call(req).await?.map_into_right_body())
    }
}

fn extract_bearer_token(req: &ServiceRequest) -> Option<String> {
    req.headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|auth| auth.strip_prefix("Bearer "))
        .map(|token| token.to_string())
}

pub async fn authentication_middleware_oauth2_cookie(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<EitherBody<BoxBody, impl MessageBody>>, Error> {
    if is_health_check(&req) || is_authenticated(&req) {
        return Ok(next.call(req).await?.map_into_right_body());
    }

    let oidc_config = get_oidc_config(&req)?;
    let oidc_redirect_uri_path = oidc_config.redirect_uri().path();

    let session = req.get_session();
    match session
        .get::<OAuthSession>(OAUTH_SESSION_KEY)
        .unwrap_or(None)
    {
        None if req.path() != oidc_redirect_uri_path => {
            let authorization_request = OAuthSecureAuthorizationRequest::code_flow(
                oidc_config.authorization_endpoint(),
                oidc_config.client_id(),
                oidc_config.scopes(),
                oidc_config.redirect_uri(),
            );

            session.insert(
                OAUTH_AUTHORIZATION_REQUEST_STATE_SESSION_KEY,
                authorization_request.state(),
            )?;

            let redirect_to_auth_server = HttpResponse::Found()
                .append_header((
                    http::header::LOCATION.to_string(),
                    Url::from(&authorization_request).to_string(),
                ))
                .finish()
                .map_into_boxed_body();

            Ok(req
                .into_response(redirect_to_auth_server)
                .map_into_left_body())
        }

        Some(oauth_session) => {
            let session_tokens = oauth_session.session_tokens();

            if oauth_session.is_access_token_expired() {
                let refresh_token_request = OAuthRefreshTokenRequest::new(
                    oidc_config.token_endpoint(),
                    oidc_config.client_id(),
                    oidc_config.client_secret(),
                    oidc_config.scopes(),
                    session_tokens.refresh_token(),
                );

                let authorization_response = match refresh_token_request.send().await {
                    Ok(authorization_response) => authorization_response,
                    Err(err) => {
                        return {
                            log::warn!("Unauthorized AUTH RESP {:?}", err);
                            Ok(req.into_response(unauthorized()).map_into_left_body())
                        }
                    }
                };

                let session_tokens = OAuthSessionTokens::new(
                    authorization_response.access_token(),
                    authorization_response.refresh_token(),
                    authorization_response.id_token(),
                    None,
                );

                if let Some(validated_tokens) = authorization_check(
                    &session_tokens,
                    oidc_config.jwks(),
                    oidc_config.client_id(),
                )
                .await
                {
                    session.insert(OAUTH_SESSION_KEY, OAuthSession::from(&validated_tokens))?;
                    session.insert(
                        USER_SESSION_KEY,
                        AuthenticatedUser::from(validated_tokens.id_token_claims()),
                    )?;
                } else {
                    log::warn!("Unauthorized");
                    return Ok(req.into_response(unauthorized()).map_into_left_body());
                }
            }

            req.extensions_mut()
                .insert(AuthenticationMethod::OAuthCodeFlow);
            Ok(next.call(req).await?.map_into_right_body())
        }

        _ => Ok(next.call(req).await?.map_into_right_body()),
    }
}

fn unauthorized() -> HttpResponse {
    HttpResponse::Unauthorized().finish().map_into_boxed_body()
}

fn get_oidc_config(req: &ServiceRequest) -> Result<&OidcConfig, Error> {
    req.app_data::<web::Data<Config>>()
        .map(|config| config.oidc_config())
        .ok_or_else(move || {
            log::error!("Failed to retrieve configuration from app data");
            actix_web::error::ErrorInternalServerError("Configuration missing")
        })
}

fn is_authenticated(req: &ServiceRequest) -> bool {
    req.extensions_mut().get::<AuthenticationMethod>().is_some()
}

fn is_health_check(req: &ServiceRequest) -> bool {
    req.path() == routes::health_check::HEALTH_CHECK_ROUTE
}

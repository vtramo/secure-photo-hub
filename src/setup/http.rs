use actix_session::config::BrowserSession;
use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::{Key, SameSite};
use actix_web::middleware::{from_fn, Logger};
use actix_web::{web, App, HttpServer};
use anyhow::Context;
use aws_sdk_s3::Client;

use crate::setup::Config;
use crate::{routes, security};

pub async fn init_http_server(config: Config) -> anyhow::Result<()> {
    log::info!("Init http server...");

    let redis_connection_string = config.redis_config.connection_string();
    let store = RedisSessionStore::new(redis_connection_string.to_string())
        .await
        .context("Failed to connect to Redis for session management")?;
    let oauth_redirect_uri_path = config.oidc_config().redirect_uri().path().to_string();

    let aws_s3_client = Client::new(&config.aws_config);
    // aws_s3_client.put_object()
    //     .bucket("")
    //     .key("a")
    //     .body(ByteStream::from())

    let server_port = config.server_port;
    HttpServer::new(move || {
        App::new()
            .wrap(from_fn(
                security::auth::middleware::authentication_middleware_oauth2_cookie,
            ))
            .wrap(from_fn(
                security::auth::middleware::authentication_middleware_bearer_token,
            ))
            .wrap(
                SessionMiddleware::builder(store.clone(), Key::from(&[0; 64]))
                    .cookie_http_only(true)
                    .cookie_secure(false)
                    .cookie_path("/".to_string())
                    .cookie_same_site(SameSite::Lax)
                    .session_lifecycle(BrowserSession::default())
                    .build(),
            )
            .wrap(Logger::new(
                "%r - %a - %{User-Agent}i - Response Status Code: %s",
            ))
            .app_data(web::Data::new(config.clone()))
            .route(
                &oauth_redirect_uri_path,
                web::get().to(security::auth::oauth::oidc_redirect_endpoint),
            )
            .route(
                routes::health_check::HEALTH_CHECK_ROUTE,
                web::get().to(routes::health_check::health_check),
            )
            .service(routes::home)
            .service(routes::post_photos)
    })
    .bind(("0.0.0.0", server_port))?
    .run()
    .await?;

    Ok(())
}

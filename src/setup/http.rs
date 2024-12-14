use std::sync::Arc;
use actix_session::config::BrowserSession;
use actix_session::storage::{RedisSessionStore};
use actix_session::SessionMiddleware;
use actix_web::cookie::{Key, SameSite};
use actix_web::middleware::{from_fn, Logger};
use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use anyhow::Context;

use crate::setup::Config;
use crate::{PhotoService, repository, routes, security, service};
use crate::repository::image_repository::AwsS3Client;
use crate::repository::PostgresDatabase;

#[derive(Debug, Clone)]
pub struct AppState<PS: PhotoService> {
    photo_service: Arc<PS>,
}

impl<PS> AppState<PS> where PS: PhotoService {
    pub fn photo_service(&self) -> Arc<PS> {
        self.photo_service.clone()
    }
}

pub async fn create_http_server(config: Config) -> anyhow::Result<Server> {
    log::info!("Init http server...");

    let redis_connection_string = config.redis_config.connection_string();
    let store = RedisSessionStore::new(redis_connection_string.to_string())
        .await
        .context("Failed to connect to Redis for session management")?;
    let oauth_redirect_uri_path = config.oidc_config().redirect_uri().path().to_string();

    let aws_s3_client = AwsS3Client::new(&config.aws_s3_config);
    let database = repository::PostgresDatabase::connect_with_db_config(&config.database_config).await?;
    let photo_service = service::photo::Service::new(database, aws_s3_client);
    let app_state = AppState { photo_service: Arc::new(photo_service) };

    let server_port = config.server_port;
    let server = HttpServer::new(move || {
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
            .app_data(web::Data::new(app_state.clone()))
            .route(
                &oauth_redirect_uri_path,
                web::get().to(security::auth::oauth::oidc_redirect_endpoint),
            )
            .route(
                routes::health_check::HEALTH_CHECK_ROUTE,
                web::get().to(routes::health_check::health_check),
            )
            .route(
                "/photos",
                web::post().to(routes::photo::post_photos::<service::photo::Service<PostgresDatabase, AwsS3Client>>)
            )
            .service(routes::home)
    })
        .bind(("0.0.0.0", server_port))?
        .run();

    Ok(server)
}

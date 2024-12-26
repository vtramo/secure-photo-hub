use std::sync::Arc;
use actix_session::config::BrowserSession;
use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::{Key, SameSite};
use actix_web::middleware::{from_fn, Logger};
use actix_web::{App, HttpServer, web};
use actix_web::dev::Server;
use anyhow::Context;

use crate::setup::Config;
use crate::{repository, routes, security, service};
use crate::service::image_storage::AwsS3Client;
use crate::repository::PostgresDatabase;
use crate::service::{AlbumService, PhotoService};

#[derive(Debug, Clone)]
pub struct PhotoRoutesState<PS: PhotoService> {
    photo_service: Arc<PS>,
}

impl<PS> PhotoRoutesState<PS> where PS: PhotoService {
    pub fn photo_service(&self) -> Arc<PS> {
        self.photo_service.clone()
    }
}

#[derive(Debug, Clone)]
pub struct AlbumRoutesState<AS: AlbumService> {
    album_service: Arc<AS>,
}

impl<AS: AlbumService> AlbumRoutesState<AS> {
    pub fn album_service(&self) -> Arc<AS> {
        self.album_service.clone()
    }
}

#[derive(Debug, Clone)]
pub struct ImageRoutesState<IS> {
    image_service: Arc<IS>
}

impl<IS> ImageRoutesState<IS> {
    pub fn image_service(&self) -> &Arc<IS> {
        &self.image_service
    }
}


pub async fn create_http_server(config: Config) -> anyhow::Result<Server> {
    log::info!("Init http server...");

    let redis_connection_string = config.redis_config.connection_string();
    let store = RedisSessionStore::new(redis_connection_string.to_string())
        .await
        .context("Failed to connect to Redis for session management")?;
    let oauth_redirect_uri_path = config.oidc_config().redirect_uri().path().to_string();

    let aws_s3_client = Arc::new(AwsS3Client::new(&config.aws_s3_config));
    let database = Arc::new(repository::PostgresDatabase::connect_with_db_config(&config.database_config).await?);
    let photo_service = service::photo::PhotoServiceImpl::new(Arc::clone(&database), Arc::clone(&aws_s3_client));
    let album_service = service::album::AlbumServiceImpl::new(Arc::clone(&database), Arc::clone(&aws_s3_client));
    let image_service = service::image::ImageServiceImpl::new(Arc::clone(&database), Arc::clone(&aws_s3_client));
    let photo_routes_state = PhotoRoutesState { photo_service: Arc::new(photo_service) };
    let album_routes_state = AlbumRoutesState { album_service: Arc::new(album_service) };
    let image_routes_state = ImageRoutesState { image_service: Arc::new(image_service) };
    
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
            .app_data(web::Data::new(photo_routes_state.clone()))
            .app_data(web::Data::new(album_routes_state.clone()))
            .app_data(web::Data::new(image_routes_state.clone()))
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
                web::post().to(routes::photo::post_photos::<service::photo::PhotoServiceImpl<PostgresDatabase, AwsS3Client>>),
            )
            .route(
                "/photos",
                web::get().to(routes::photo::get_photos::<service::photo::PhotoServiceImpl<PostgresDatabase, AwsS3Client>>),
            )
            .route(
                "/photos/{id}",
                web::get().to(routes::photo::get_photo_by_id::<service::photo::PhotoServiceImpl<PostgresDatabase, AwsS3Client>>),
            )
            .route(
                "/photos/{id}",
                web::patch().to(routes::photo::patch_photo::<service::photo::PhotoServiceImpl<PostgresDatabase, AwsS3Client>>),
            )
            .route(
                "/albums",
                web::get().to(routes::album::get_albums::<service::album::AlbumServiceImpl<PostgresDatabase, AwsS3Client>>),
            )
            .route(
                "/albums/{id}",
                web::get().to(routes::album::get_album_by_id::<service::album::AlbumServiceImpl<PostgresDatabase, AwsS3Client>>),
            )
            .route(
                "/albums",
                web::post().to(routes::album::post_albums::<service::album::AlbumServiceImpl<PostgresDatabase, AwsS3Client>>),
            )
            .route(
                "/images/{id}",
                web::get().to(routes::image::get_image_by_id::<service::image::ImageServiceImpl<PostgresDatabase, AwsS3Client>>),
            )
            .service(routes::home)
    })
        .bind(("0.0.0.0", server_port))?
        .run();

    Ok(server)
}

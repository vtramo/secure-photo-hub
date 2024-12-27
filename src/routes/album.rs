use actix_multipart::form::MultipartForm;
use actix_web::{HttpResponse, Responder, web};
use uuid::Uuid;

use crate::models::api::album::{AlbumApi, CreateAlbumApi};
use crate::models::service::album::CreateAlbumWithCover;
use crate::security::auth::user::AuthenticatedUser;
use crate::service::AlbumService;
use crate::setup::AlbumRoutesState;

pub const ALBUMS_ROUTE: &'static str = "/albums";
pub const ALBUM_BY_ID_ROUTE: &'static str = "/albums/{id}";

pub async fn post_albums<AS: AlbumService>(
    authenticated_user: AuthenticatedUser,
    MultipartForm(create_album_api): MultipartForm<CreateAlbumApi>,
    app_state: web::Data<AlbumRoutesState<AS>>,
) -> impl Responder {
    let create_album_with_cover = CreateAlbumWithCover::try_from(create_album_api).unwrap();

    let album_api = AlbumApi::from(app_state
        .get_ref()
        .album_service()
        .create_album(&authenticated_user, &create_album_with_cover)
        .await
        .unwrap());  // TODO: error handling
    
    HttpResponse::Created().json(album_api)
}

pub async fn get_album_by_id<AS: AlbumService>(
    authenticated_user: AuthenticatedUser,
    id: web::Path<Uuid>,
    app_state: web::Data<AlbumRoutesState<AS>>,
) -> impl Responder {
    app_state
        .get_ref()
        .album_service()
        .get_album_by_id(&authenticated_user, &id.into_inner())
        .await
        .unwrap() // TODO: error handling
        .map(|album| HttpResponse::Ok().json(AlbumApi::from(album)))
        .unwrap_or(HttpResponse::NotFound().finish())
}

pub async fn get_albums<AS: AlbumService>(
    authenticated_user: AuthenticatedUser,
    app_state: web::Data<AlbumRoutesState<AS>>,
) -> impl Responder {
    let albums = app_state
        .get_ref()
        .album_service()
        .get_all_albums(&authenticated_user)
        .await
        .unwrap()
        .map::<AlbumApi>();
    
    HttpResponse::Ok().json(albums)
}
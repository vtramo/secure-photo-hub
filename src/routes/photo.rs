use actix_multipart::form::MultipartForm;
use actix_web::{HttpResponse, Responder, web};
use uuid::Uuid;

use crate::models::api::photo::{PatchPhotoApi, PhotoApi};
use crate::models::api::photo::UploadPhotoApi;
use crate::models::service::photo::{UpdatePhoto, UploadPhoto};
use crate::service::PhotoService;
use crate::security::auth::user::AuthenticatedUser;
use crate::setup::PhotoRoutesState;

pub const PHOTOS_ROUTE: &'static str = "/photos";
pub const PHOTO_BY_ID_ROUTE: &'static str = "/photos/{id}";

pub async fn post_photos<PS: PhotoService>(
    authenticated_user: AuthenticatedUser,
    MultipartForm(upload_photo_api): MultipartForm<UploadPhotoApi>,
    app_state: web::Data<PhotoRoutesState<PS>>,
) -> impl Responder {
    let upload_photo = UploadPhoto::try_from(upload_photo_api).unwrap(); // TODO: error handling

    let photo = PhotoApi::from(app_state
        .get_ref()
        .photo_service()
        .create_photo(&authenticated_user, &upload_photo)
        .await
        .unwrap()); // TODO: error handling

    HttpResponse::Created().json(photo)
}

pub async fn get_photos<PS: PhotoService>(
    authenticated_user: AuthenticatedUser,
    app_state: web::Data<PhotoRoutesState<PS>>,
) -> impl Responder {

    let photos = app_state
        .get_ref()
        .photo_service()
        .get_all_photos(&authenticated_user)
        .await
        .unwrap() // TODO: error handling
        .map::<PhotoApi>();
    
    HttpResponse::Ok().json(photos)
}

pub async fn get_photo_by_id<PS: PhotoService>(
    authenticated_user: AuthenticatedUser,
    id: web::Path<Uuid>,
    app_state: web::Data<PhotoRoutesState<PS>>,
) -> impl Responder {
    app_state
        .get_ref()
        .photo_service()
        .get_photo_by_id(&authenticated_user, &id.into_inner())
        .await
        .unwrap() // TODO: error handling
        .map(|photo| HttpResponse::Ok().json(PhotoApi::from(photo)))
        .unwrap_or(HttpResponse::NotFound().finish())
}

pub async fn patch_photo<PS: PhotoService>(
    authenticated_user: AuthenticatedUser,
    photo_id: web::Path<Uuid>,
    patch_photo_api: web::Json<PatchPhotoApi>,
    app_state: web::Data<PhotoRoutesState<PS>>,
) -> impl Responder {
    app_state
        .get_ref()
        .photo_service()
        .update_photo(&authenticated_user, &UpdatePhoto::from(photo_id.into_inner(), patch_photo_api.into_inner()))
        .await
        .map(|photo| HttpResponse::Ok().json(PhotoApi::from(photo)))
        .unwrap_or(HttpResponse::NotFound().finish()) // TODO: error handling
}
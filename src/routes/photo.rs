use actix_multipart::form::MultipartForm;
use actix_web::{Responder, HttpResponse, web};
use crate::models::api::photo::PhotoApi;
use crate::models::api::photo::UploadPhotoApi;
use crate::models::service::photo::{UploadPhoto};
use crate::PhotoService;
use crate::security::auth::user::AuthenticatedUser;
use crate::setup::AppState;


pub async fn post_photos<PS: PhotoService>(
    authenticated_user: AuthenticatedUser,
    MultipartForm(upload_photo_api): MultipartForm<UploadPhotoApi>,
    app_state: web::Data<AppState<PS>>,
) -> impl Responder {
    let upload_photo = UploadPhoto::try_from(upload_photo_api).unwrap();

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
    app_state: web::Data<AppState<PS>>,
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
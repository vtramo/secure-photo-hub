use actix_multipart::form::MultipartForm;
use actix_web::{Responder, HttpResponse, web};
use crate::models::api::UploadPhotoApi;
use crate::models::service::photo::{UploadImage, UploadPhoto};
use crate::PhotoService;
use crate::security::auth::user::User;
use crate::setup::AppState;


pub async fn post_photos<PS: PhotoService>(
    user: User,
    MultipartForm(upload_photo_api): MultipartForm<UploadPhotoApi>,
    app_state: web::Data<AppState<PS>>,
) -> impl Responder {
    let upload_photo = UploadPhoto::try_from(upload_photo_api).unwrap();

    let photo = app_state
        .get_ref()
        .photo_service()
        .create_photo(upload_photo)
        .await
        .unwrap();

    HttpResponse::Created().json(photo.title())
}

use actix_multipart::form::MultipartForm;
use actix_web::{post, Responder, HttpResponse};
use crate::models::api::UploadPhotoApi;
use crate::models::service::photo::{UploadImage};
use crate::security::auth::user::User;

#[post("/photos")]
pub async fn post_photos(
    user: User,
    MultipartForm(form): MultipartForm<UploadPhotoApi>
) -> impl Responder {
    let UploadPhotoApi { 
        file: mut temp_file,
        metadata: mut metadata,
    } = form;

    dbg!(temp_file.size, &temp_file.content_type, &temp_file.file_name);
    match UploadImage::try_from(temp_file) {
        Ok(upload_image) => {},
        Err(err) => return HttpResponse::BadRequest().body(format!("error: {:?}", err)), // TODO: api error handling
    };

    HttpResponse::NoContent().finish()
}

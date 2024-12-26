use actix_web::{HttpResponse, Responder, web};
use mime::PNG;
use uuid::Uuid;
use crate::models::api::ImageTransformOptionsApi;

use crate::models::service::image::ImageTransformOptions;
use crate::security::auth::user::AuthenticatedUser;
use crate::service::ImageService;
use crate::setup::ImageRoutesState;

pub async fn get_image_by_id<IS: ImageService>(
    authenticated_user: AuthenticatedUser,
    id: web::Path<Uuid>,
    convert_options: web::Query<ImageTransformOptionsApi>,
    app_state: web::Data<ImageRoutesState<IS>>,
) -> impl Responder {
    let convert_options = convert_options.into_inner();
    app_state
        .get_ref()
        .image_service()
        .get_image(&authenticated_user, &id.into_inner(), &ImageTransformOptions::from(convert_options))
        .await
        .unwrap() // TODO: error handling
        .map(|image| HttpResponse::Ok().content_type(PNG.to_string()).body(image.take_bytes()))
        .unwrap_or(HttpResponse::NotFound().finish())
}
use actix_web::{get, HttpResponse, Responder};

use crate::security;

pub mod health_check;
pub mod photo;
pub mod album;
pub mod image;


#[get("/")]
pub async fn home(user: security::auth::user::AuthenticatedUser) -> impl Responder {
    HttpResponse::Ok().body(format!("{:#?}", user))
}
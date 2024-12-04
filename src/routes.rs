use actix_web::{get, HttpResponse, Responder};

use crate::security;

pub mod health_check;
pub mod photo;


#[get("/")]
pub async fn home(user: security::auth::user::User) -> impl Responder {
    HttpResponse::Ok().body(format!("{:#?}", user))
}
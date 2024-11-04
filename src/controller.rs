use actix_web::{get, HttpResponse, Responder};

use crate::security;

#[get("/")]
pub async fn home(user: security::auth::user::User) -> impl Responder {
    HttpResponse::Ok()
        .body(format!("{:#?}", user))
}
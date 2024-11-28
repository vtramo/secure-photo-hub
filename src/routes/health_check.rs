use actix_web::{HttpResponse, Responder};

pub const HEALTH_CHECK_ROUTE: &'static str = "/healthcheck";

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

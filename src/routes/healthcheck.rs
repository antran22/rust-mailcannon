use actix_web::{HttpResponse, Responder};

pub async fn check() -> impl Responder {
    HttpResponse::Ok().finish()
}

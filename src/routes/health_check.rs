use actix_web::HttpResponse;
use actix_web::{get, Responder};

#[get("/health_check")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

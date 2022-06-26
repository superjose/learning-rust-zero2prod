use actix_web::{dev::Server, get, post, web::Form, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::net::TcpListener;

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String,
}

#[post("/subscriptions")]
async fn create_subscription(_form: Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(health_check)
            .service(create_subscription)
        // .route("/hey", web::get().to(manual_hello))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

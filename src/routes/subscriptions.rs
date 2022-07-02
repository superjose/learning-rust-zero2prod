use actix_web::{post, web::Form, HttpResponse};

#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String,
}

#[post("/subscriptions")]
async fn create_subscription(_form: Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

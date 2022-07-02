use actix_web::{
    post,
    web::{self, Form},
    HttpResponse,
};
use sqlx::PgConnection;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

#[post("/subscriptions")]
async fn create_subscription(_form: Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn subscribe(
    _form: web::Form<FormData>,
    _connection: web::Data<PgConnection>,
) -> HttpResponse {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new::v4(),
        _form.email,
        _form.name,
        Utc::now,
    )
    .execute(_connection.get_ref())
    .await;

    HttpResponse::Ok().finish()
}

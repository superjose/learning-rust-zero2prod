use actix_web::{
    get, post,
    web::{self},
    HttpResponse,
};
use chrono::{DateTime, Utc};
use sqlx::{types, PgPool};

use uuid::Uuid;
#[derive(serde::Serialize, Debug)]
pub struct Subscriber {
    // You had to enable serde for this to work
    id: types::Uuid,
    name: String,
    email: String,
    // You had to enable serde for this to work
    subscribed_at: DateTime<Utc>,
}

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        request_id = %Uuid::new_v4(),
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
#[post("/subscriptions")]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let query_result = insert_subscriber(&pool, &form).await;
    match query_result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => {
            // Note that we are logging the debug statement {:?}
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, pool)
)]
pub async fn insert_subscriber(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(())
}

#[tracing::instrument(name = "Getting All the Subscribers", skip(pool))]
#[get("/subscribers")]
pub async fn get_subscribers(pool: web::Data<PgPool>) -> HttpResponse {
    let query_result = subscribers(&pool).await;
    println!("{:?}", query_result);
    match query_result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(name = "Getting subscribers from the database", skip(pool))]
async fn subscribers(pool: &PgPool) -> Result<Vec<Subscriber>, sqlx::Error> {
    let query_result = sqlx::query_as!(
        Subscriber,
        r#"
            SELECT * FROM subscriptions
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute get_subscribers: {:?}", e);
        e
    })?;

    Ok(query_result)
}

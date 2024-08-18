use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::types::chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(
    form: web::Form<FormData>,
    connection_pool: web::Data<PgPool>,
) -> impl Responder {
    match sqlx::query!(
        "insert into subscriptions (id, email, name, subscribed_at) values ($1, $2, $3, $4)",
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(connection_pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok(),
        Err(e) => {
            println!("Failed to execute query {}", e);
            HttpResponse::InternalServerError()
        }
    }
}

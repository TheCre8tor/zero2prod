//! src/routes/subscriptions

use actix_web::web::{Data, Form};
use actix_web::HttpResponse;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: Form<FormData>, connection: Data<PgPool>) -> HttpResponse {
    let query = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(connection.get_ref())
    .await;

    match query {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(error) => {
            println!("Failed to execute query: {}", error);
            HttpResponse::InternalServerError().finish()
        }
    }
}

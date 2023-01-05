use actix_web::{post, HttpResponse};

#[post("/newsletters")]
pub async fn publish_newsletter() -> HttpResponse {
    HttpResponse::Ok().finish()
}

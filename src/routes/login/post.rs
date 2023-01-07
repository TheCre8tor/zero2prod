//! src/routes/login/login_api.rs

use actix_web::{post, HttpResponse};

#[post("/login")]
pub async fn login() -> HttpResponse {
    HttpResponse::Ok().finish()
}

//!  src/routes/login/mod.rs

use actix_web::http::header::ContentType;
use actix_web::{get, HttpResponse};

#[get("/login")]
pub async fn login_form() -> HttpResponse {
    let read_file = include_str!("login.html");
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(read_file)
}
//! src/routes/login/login_api.rs

use actix_web::http::header::LOCATION;
use actix_web::{post, web, HttpResponse};
use secrecy::Secret;

#[derive(serde::Deserialize, Debug)]
pub struct FormData {
    username: String,
    password: Secret<String>,
}

#[post("/login")]
pub async fn login(form: web::Form<FormData>) -> HttpResponse {
    dbg!("{:?}", form);

    HttpResponse::SeeOther()
        .insert_header((LOCATION, "/"))
        .finish()
}

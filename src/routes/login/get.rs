//!  src/routes/login/mod.rs

use actix_web::http::header::ContentType;
use actix_web::{get, web, HttpResponse};
use tera::{escape_html, Tera};

#[derive(serde::Deserialize)]
pub struct QueryParams {
    error: Option<String>,
}

#[get("/login")]
pub async fn login_form(query: web::Query<QueryParams>) -> HttpResponse {
    let read_file = include_str!("login.html");

    let error_html = match query.0.error {
        Some(message) => escape_html(&message),
        None => "".into(),
    };

    let mut tera = Tera::default();
    tera.add_raw_template("login", read_file).unwrap();

    let mut ctx = tera::Context::new();
    ctx.insert("error", &error_html);

    let tera = match tera.render("login", &ctx) {
        Ok(template) => template,
        Err(_) => "".into(),
    };

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(tera)
}

//!  src/routes/login/mod.rs

use actix_web::http::header::ContentType;
use actix_web::{get, HttpRequest, HttpResponse};
use tera::Tera;

#[tracing::instrument(name = "Rendering Login Form")]
#[get("/login")]
pub async fn login_form(request: HttpRequest) -> HttpResponse {
    let read_file = include_str!("login.html");

    let error_html = match request.cookie("_flash") {
        None => "".into(),
        Some(cookie) => cookie.value().to_string(),
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

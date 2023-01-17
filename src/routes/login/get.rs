//!  src/routes/login/mod.rs

use actix_web::http::header::ContentType;
use actix_web::{get, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;
use tera::Tera;

#[tracing::instrument(name = "Rendering Login Form", skip(flash_messages))]
#[get("/login")]
pub async fn login_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let read_file = include_str!("login.html");

    let mut error_html = String::new();

    for message in flash_messages.iter() {
        writeln!(error_html, "{}", message.content()).unwrap();
    }

    let mut tera = Tera::default();
    tera.add_raw_template("login", read_file).unwrap();

    let mut ctx = tera::Context::new();
    ctx.insert("error", &error_html.trim());

    let tera = match tera.render("login", &ctx) {
        Ok(template) => template,
        Err(_) => "".into(),
    };

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(tera)
}

use actix_web::{http::header::ContentType, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;
use tera::Tera;

#[tracing::instrument(name = "Change password form", skip(flash_message))]
pub async fn change_password_form(
    flash_message: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut msg_html = String::new();

    for message in flash_message.iter() {
        writeln!(msg_html, "{}", message.content()).unwrap();
    }

    let read_file = include_str!("password.html");

    let mut tera = Tera::default();
    tera.add_raw_template("password", read_file).unwrap();

    let mut ctx = tera::Context::new();
    ctx.insert("error", &msg_html.trim());

    let tera = match tera.render("password", &ctx) {
        Ok(template) => template,
        Err(_) => "".into(),
    };

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(tera))
}

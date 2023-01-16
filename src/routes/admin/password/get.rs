use actix_web::{get, http::header::ContentType, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;
use tera::Tera;

use crate::{
    session_state::TypedSession,
    utils::{error500, see_other},
};

#[tracing::instrument(name = "Change password form", skip(session, flash_message))]
#[get("/admin/password")]
pub async fn change_password_form(
    session: TypedSession,
    flash_message: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(error500)?.is_none() {
        return Ok(see_other("/login"));
    }

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

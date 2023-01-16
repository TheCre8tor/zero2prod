use actix_web::{get, http::header::ContentType, HttpResponse};
use tera::Tera;

use crate::{
    session_state::TypedSession,
    utils::{error500, see_other},
};

#[tracing::instrument(name = "Change password form", skip(session))]
#[get("/admin/password")]
pub async fn change_password_form(session: TypedSession) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(error500)?.is_none() {
        return Ok(see_other("/login"));
    }

    let read_file = include_str!("password.html");

    let mut tera = Tera::default();
    tera.add_raw_template("password", read_file).unwrap();

    let ctx = tera::Context::new();

    let tera = match tera.render("password", &ctx) {
        Ok(template) => template,
        Err(_) => "".into(),
    };

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(tera))
}

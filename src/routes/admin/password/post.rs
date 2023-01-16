use actix_web::{post, web, HttpResponse};
use secrecy::Secret;

use crate::{
    session_state::TypedSession,
    utils::{error500, see_other},
};

#[derive(serde::Deserialize)]
pub struct FormData {
    current_password: Secret<String>,
    new_password: Secret<String>,
    new_password_check: Secret<String>,
}

#[post("/admin/password")]
pub async fn change_password(
    _form: web::Form<FormData>,
    session: TypedSession,
) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(error500)?.is_none() {
        return Ok(see_other("/login"));
    }

    Ok(HttpResponse::Ok().finish())
}

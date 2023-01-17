use actix_web::{post, web, HttpResponse};
use actix_web_flash_messages::FlashMessage;
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;

use crate::authentication::{validate_credentials, AuthError, Credentials};

use crate::{
    routes::admin::dashboard::get_username,
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
    form: web::Form<FormData>,
    session: TypedSession,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = session.get_user_id().map_err(error500)?;
    if user_id.is_none() {
        return Ok(see_other("/login"));
    }

    let user_id = user_id.unwrap();

    if form.new_password.expose_secret().len() < 12 {
        FlashMessage::warning("Password should be greater than 12").send();
        return Ok(see_other("/admin/password"));
    }

    if form.new_password.expose_secret() != form.new_password_check.expose_secret() {
        FlashMessage::error(
            "You entered two different new passwords - the field values must match.",
        )
        .send();

        return Ok(see_other("/admin/password"));
    }

    let username = get_username(user_id, &pool).await.map_err(error500)?;

    let credentials = Credentials {
        username,
        password: form.0.current_password,
    };

    if let Err(error) = validate_credentials(credentials, &pool).await {
        return match error {
            AuthError::InvalidCredentials(_) => {
                FlashMessage::error("The current password is incorrect.").send();
                Ok(see_other("/admin/password"))
            }
            AuthError::UnexpectedError(_) => Err(error500(error)),
        };
    }

    crate::authentication::change_password(user_id, form.0.new_password, &pool)
        .await
        .map_err(error500)?;

    FlashMessage::error("Your password has been changed.").send();

    Ok(see_other("/admin/password"))
}

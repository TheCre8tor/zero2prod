//! src/routes/login/login_api.rs

use actix_session::Session;
use actix_web::error::InternalError;
use actix_web::http::header::LOCATION;
use actix_web::{post, web, HttpResponse};

use actix_web_flash_messages::FlashMessage;
use secrecy::Secret;
use sqlx::PgPool;

use crate::authentication::{validate_credentials, AuthError, Credentials};
use crate::routes::error_chain_fmt;

#[derive(serde::Deserialize, Debug)]
pub struct FormData {
    username: String,
    password: Secret<String>,
}

#[tracing::instrument(
    skip(form, pool, session),
    fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
)]
#[post("/login")]
pub async fn login(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
    session: Session,
) -> Result<HttpResponse, InternalError<LoginError>> {
    let credentials = Credentials {
        username: form.0.username,
        password: form.0.password,
    };

    tracing::Span::current().record("username", &tracing::field::display(&credentials.username));
    match validate_credentials(credentials, &pool).await {
        Ok(user_id) => {
            tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
            session.renew();
            // if error occurs while storing the session, redirect to login page
            session
                .insert("user_id", user_id)
                .map_err(|error| login_redirect(LoginError::UnexpectedError(error.into())))?;
            // if successful, redirect to the admin dashboard page
            Ok(HttpResponse::SeeOther()
                .insert_header((LOCATION, "/admin/dashboard"))
                .finish())
        }
        Err(error) => {
            let error = match error {
                AuthError::InvalidCredentials(_) => LoginError::AuthError(error.into()),
                AuthError::UnexpectedError(_) => LoginError::UnexpectedError(error.into()),
            };

            FlashMessage::error(error.to_string()).send();
            let response = HttpResponse::SeeOther()
                .insert_header((LOCATION, "/login"))
                // We no longer send cookie directly anylonger
                // we send cookie through FlashMessage ->
                // The FlashMessagesFramework middleware takes care of all
                // the heavy-lifting behind the scenes - creating the cookie,
                // signing it, setting the right properties, etc.
                // .cookie(Cookie::new("_flash", error.to_string()))
                .finish();

            Err(InternalError::from_response(error, response))
        }
    }
}

#[derive(thiserror::Error)]
pub enum LoginError {
    #[error("Authentication failed")]
    AuthError(#[source] anyhow::Error),
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

fn login_redirect(error: LoginError) -> InternalError<LoginError> {
    FlashMessage::error(error.to_string()).send();

    let response = HttpResponse::SeeOther()
        .insert_header((LOCATION, "/login"))
        .finish();

    InternalError::from_response(error, response)
}

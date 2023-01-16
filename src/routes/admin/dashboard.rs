//! src/routes/admin/dashboard.rs

use actix_web::{get, http::header::ContentType, web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;
use tera::Tera;
use uuid::Uuid;

use crate::session_state::TypedSession;

fn error500<T>(error: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    actix_web::error::ErrorInternalServerError(error)
}

#[get("/admin/dashboard")]
pub async fn admin_dashboard(
    pool: web::Data<PgPool>,
    session: TypedSession,
) -> Result<HttpResponse, actix_web::Error> {
    let username = if let Some(user_id) = session.get_user_id().map_err(error500)? {
        get_username(user_id, &pool).await.map_err(error500)?
    } else {
        todo!()
    };

    let read_file = include_str!("dashboard.html");

    let mut tera = Tera::default();
    tera.add_raw_template("dashboard", read_file).unwrap();

    let mut ctx = tera::Context::new();
    ctx.insert("username", &username);

    let tera = match tera.render("dashboard", &ctx) {
        Ok(template) => template,
        Err(_) => "".into(),
    };

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(tera))
}

#[tracing::instrument(name = "Get username", skip(pool))]
async fn get_username(user_id: Uuid, pool: &PgPool) -> Result<String, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        SELECT username
        FROM users
        WHERE user_id = $1
        "#,
        user_id,
    )
    .fetch_one(pool)
    .await
    .context("Failed to perform a query to retrieve a username.")?;

    Ok(row.username)
}

//! src/routes/admin/dashboard.rs

use actix_web::{http::header::ContentType, web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;
use tera::Tera;
use uuid::Uuid;

use crate::{authentication::UserId, utils::error500};

pub async fn admin_dashboard(
    pool: web::Data<PgPool>,
    user_id: web::ReqData<UserId>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id.into_inner();
    let username = get_username(*user_id, &pool).await.map_err(error500)?;

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
pub async fn get_username(user_id: Uuid, pool: &PgPool) -> Result<String, anyhow::Error> {
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

//!  src/routes/login/mod.rs

use actix_web::http::header::ContentType;
use actix_web::{get, web, HttpResponse};
use hmac::{Hmac, Mac};
use secrecy::ExposeSecret;
use tera::{escape_html, Tera};

use crate::startup::HmacSecret;

#[derive(serde::Deserialize)]
pub struct QueryParams {
    error: String,
    tag: String,
}

#[tracing::instrument(name = "Rendering Login Form", skip(query, secret))]
#[get("/login")]
pub async fn login_form(
    query: Option<web::Query<QueryParams>>,
    secret: web::Data<HmacSecret>,
) -> HttpResponse {
    let read_file = include_str!("login.html");

    let error_html = match query {
        None => "".into(),
        Some(query_params) => match query_params.0.verify(&secret) {
            Ok(error_msg) => escape_html(&error_msg),
            Err(error) => {
                tracing::warn!(
                    error.message = %error,
                    error.cause_chain = ?error,
                    "Failed to verify query parameters using the HMAC tag"
                );
                "".into()
            }
        },
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

impl QueryParams {
    fn verify(self, secret: &HmacSecret) -> Result<String, anyhow::Error> {
        let tag = hex::decode(self.tag)?;

        let query_string = format!("error={}", urlencoding::Encoded::new(&self.error));

        let secret = secret.0.expose_secret().as_bytes();

        let mut mac = Hmac::<sha2::Sha256>::new_from_slice(secret).unwrap();
        mac.update(query_string.as_bytes());
        mac.verify_slice(&tag)?;

        Ok(self.error)
    }
}

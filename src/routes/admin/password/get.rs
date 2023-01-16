use actix_web::{get, http::header::ContentType, HttpResponse};
use tera::Tera;

#[get("/admin/password")]
pub async fn change_password_form() -> Result<HttpResponse, actix_web::Error> {
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

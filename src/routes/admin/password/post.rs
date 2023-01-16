use actix_web::{get, HttpResponse};

#[get("/admin/password")]
pub async fn change_password() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().finish())
}

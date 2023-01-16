//! src/routes/admin/dashboard.rs

use actix_web::{get, HttpResponse};

#[get("/admin/dashboard")]
pub async fn admin_dashboard() -> HttpResponse {
    HttpResponse::Ok().finish()
}

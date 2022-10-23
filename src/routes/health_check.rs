use actix_web::{HttpRequest, HttpResponse, Responder};

pub async fn health_check(_red: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}
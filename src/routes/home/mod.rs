use actix_web::http::header::ContentType;
use actix_web::{get, HttpResponse};

#[get("/")]
pub async fn home() -> HttpResponse {
    let read_file = include_str!("home.html");
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(read_file)
}

use actix_web::{web::get, App, HttpResponse, HttpServer, Responder};

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn run() -> Result<(), std::io::Error> {
    HttpServer::new(|| App::new().route("/health_check", get().to(health_check)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}

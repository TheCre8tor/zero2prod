//! src/lib.rs

use actix_web::dev::Server;
use actix_web::web::{get, post, Form};
use actix_web::{App, HttpResponse, HttpServer};
use std::net::TcpListener; // Transmission Control Protocol: [TCP]

// NOTE: HTTP & TCP is a protocol

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn subscribe(_form: Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health-check", get().to(health_check))
            .route("/subscriptions", post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

//! src/lib.rs

use actix_web::dev::Server;
use actix_web::{web::get, App, HttpResponse, HttpServer};
use std::net::TcpListener; // Transmission Control Protocol: [TCP]

// NOTE: HTTP & TCP is a protocol

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health-check", get().to(health_check)))
        .listen(listener)?
        .run();

    Ok(server)
}

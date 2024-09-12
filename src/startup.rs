use actix_web::dev::Server;
use actix_web::web::{get, post};
use actix_web::{App, HttpServer};
use std::net::TcpListener; // Transmission Control Protocol: [TCP]

use crate::routes::{health_check, subscribe};

// NOTE: HTTP & TCP is a protocol

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

use actix_web::dev::Server;
use actix_web::web::{get, post, Data};
use actix_web::{App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener; // Transmission Control Protocol: [TCP]

use crate::routes::{health_check, subscribe};

// NOTE: HTTP & TCP is a protocol

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .route("/health-check", get().to(health_check))
            .route("/subscriptions", post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}

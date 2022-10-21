use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn health_check(_red: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

// We need to mark `run` as public.
// It is no longer a binary entrypoint, therefore we can mark it as async
// without having to use any proc-macro incantation.
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    //! HttpServer, handles all transport level concerns.
    let server = HttpServer::new(|| 
        App::new()
        .route("/health_check", web::get().to(health_check)))
        .listen(listener)?
        .run();

    Ok(server)
}

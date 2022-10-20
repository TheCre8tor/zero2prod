use actix_web::{web::{get}, App, HttpRequest, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //! HttpServer, handles all transport level concerns.
    HttpServer::new(|| {
        App::new()
            .route("/", get().to(greet))
            .route("/{name}", get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}



use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, dev::Server, web};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/{name}", web::get().to(greet))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

// curl http://127.0.0.1:8000/lol
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

//curl -v http://127.0.0.1:8000/health_check
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::net::TcpListener;

#[derive(Deserialize, Debug)]
struct Subscription {
    name: String,
    email: String,
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

async fn subscribe(_form: web::Form<Subscription>) -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/healthcheck", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
        .listen(listener)?
        .run();

    Ok(server)
}

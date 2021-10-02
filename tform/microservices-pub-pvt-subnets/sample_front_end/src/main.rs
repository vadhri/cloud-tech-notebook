use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::client::Client;
use actix_web::dev::Body;
use std::net::{SocketAddr, ToSocketAddrs};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // service discovery.
    let mut addrs_iter = "amazon.com:443".to_socket_addrs().unwrap();

    println!("{:?}", addrs_iter);
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
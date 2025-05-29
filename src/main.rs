use actix_web::{get, App, HttpServer, HttpResponse, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello over HTTPS!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up SSL builder
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| {
        App::new().service(hello)
    })
    .bind_openssl(("127.0.0.1", 8080), builder)?
    .run()
    .await
}

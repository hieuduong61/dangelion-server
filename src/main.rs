use actix_web::{get, App, HttpServer, HttpResponse, Responder};
use actix_cors::Cors;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde::Serialize;


#[derive(Serialize)]
struct MyData {
    message: String,
    code: u16,
}

#[get("/")]
async fn hello() -> impl Responder {
    let data = MyData {
        message: "Hello from Actix Web with JSON!".to_string(),
        code: 200,
    };
    HttpResponse::Ok().json(data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec!["Content-Type"])
            .supports_credentials();

        App::new()
            .wrap(cors)
            .service(hello)
    })
    .bind_openssl(("127.0.0.1", 8080), builder)?
    .run()
    .await
}

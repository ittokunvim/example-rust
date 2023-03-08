use actix_web::{http, App, HttpServer};
use actix_web::middleware::Logger;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use std::time::Duration;

mod routes;

#[rustfmt::skip]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    let app = move || {
        App::new()
            .wrap(Logger::default())
            .configure(routes::application_routes)
            .configure(routes::server_routes)
            .configure(routes::extractor_routes)
            .configure(routes::handler_routes)
            .configure(routes::error_routes)
    };

    let _one   = HttpServer::new(app).keep_alive(Duration::from_secs(75));
    let _two   = HttpServer::new(app).keep_alive(http::KeepAlive::Os);
    let _three = HttpServer::new(app).keep_alive(None);

    HttpServer::new(app)
        .workers(1)
        .bind_openssl(("127.0.0.1", 8080), builder)?
        .run()
        .await
}

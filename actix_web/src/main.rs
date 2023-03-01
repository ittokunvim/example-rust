use actix_web::{web, App, HttpResponse, HttpServer, guard, http};
use std::{sync::Mutex, time::Duration, cell::Cell};

use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod config;
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    let app = move || {
        let counter = web::Data::new(api::AppStateWithCounter {
            app_name: String::from("Actix Web"),
            counter: Mutex::new(0),
        });
        let state_counter = web::Data::new(api::StateStruct {
            count: Cell::new(0),
        });

        let www_guard = web::scope("/")
            .guard(guard::Header("Host", "www.rust-lang.org"))
            .route("", web::to(|| async { HttpResponse::Ok().body("www") }));
        let user_guard = web::scope("/")
            .guard(guard::Header("Host", "users.rust-lang.org"))
            .route("", web::to(|| async { HttpResponse::Ok().body("user") }));

        let users_scope = web::scope("/users").service(api::show_users);
        let app_scope = web::scope("/app")
            .route("/index.html", web::get().to(api::app));

        App::new()
            .configure(config::config)
            .app_data(counter)
            .app_data(config::json_config)
            .app_data(state_counter)
            .service(web::scope("/api").configure(config::api_config))
            .service(www_guard)
            .service(user_guard)
            .service(api::index)
            .service(api::hello)
            .service(api::echo)
            .service(users_scope)
            .service(app_scope)
            .route("/hey", web::get().to(api::manual_hello))
            .service(api::sleep)
            .service(api::quit)
            .service(api::extractors)
            .service(api::post_friend)
            .service(api::query)
            .service(api::json)
            .service(api::form)
            .service(api::show_count)
            .service(api::add_one)
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

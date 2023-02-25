use actix_web::{get, post, web, HttpResponse, Responder};
use std::{sync::Mutex, time::Duration};

use tokio;

pub struct AppStateWithCounter {
    pub app_name: String,
    pub counter: Mutex<i32>,
}

#[get("/")]
async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let app_name = &data.app_name;
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    format!("Hello {app_name}, Request number: {counter}")
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/show")]
async fn show_users() -> impl Responder {
    HttpResponse::Ok().body("Alice, Bob, Chris, Dan, Eve")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub async fn app() -> impl Responder {
    "Hello world!"
}

#[get("/sleep")]
pub async fn sleep() -> impl Responder {
    tokio::time::sleep(Duration::from_secs(5)).await;
    "response"
}

#[get("/quit")]
pub async fn quit() -> HttpResponse {
    HttpResponse::Ok()
        // .connection_type(http::ConnectionType::Close)
        .force_close()
        .finish()
}

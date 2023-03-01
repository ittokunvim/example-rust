use actix_web::{get, post, web, Result, Responder, HttpRequest, HttpResponse};
use std::{sync::Mutex, time::Duration, cell::Cell};
use serde::Deserialize;

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
async fn sleep() -> impl Responder {
    tokio::time::sleep(Duration::from_secs(5)).await;
    "response"
}

#[get("/quit")]
async fn quit() -> HttpResponse {
    HttpResponse::Ok()
        // .connection_type(http::ConnectionType::Close)
        .force_close()
        .finish()
}

#[derive(Deserialize)]
pub struct Extractors {
    pub id: u32,
    pub username: String,
}

#[get("/extractors")]
async fn extractors(path: web::Path<(String, String)>, info: web::Json<Extractors>) -> impl Responder {
    let path = path.into_inner();
    format!("{} {} {} {}", path.0, path.1, info.id, info.username)
}

#[derive(Deserialize)]
pub struct PostInfo {
    pub post_id: u32,
    pub friend: String,
}

// #[get("/posts/{post_id}/{friend}")]
// async fn post_friend(info: web::Path<PostInfo>) -> Result<String> {
//     Ok(format!(
//         "Welcome {}, user_id: {}",
//         info.friend, info.post_id
//     ))
// }

#[get("/posts/{post_id}/{friend}")]
async fn post_friend(req: HttpRequest) -> Result<String> {
    let name: String = req.match_info().get("friend").unwrap().parse().unwrap();
    let postid: i32 = req.match_info().query("post_id").parse().unwrap();

    Ok(format!("Welcome {}, post_id: {}", name, postid))
}

#[derive(Deserialize)]
struct QueryStruct {
    name: String,
}

#[get("/query")]
async fn query(info: web::Query<QueryStruct>) -> String {
    format!("Welcome {}", info.name)
}

#[derive(Deserialize)]
struct JsonStruct {
    name: String,
}

#[post("/json")]
async fn json(info: web::Json<JsonStruct>) -> Result<String> {
    Ok(format!("Welcome {}", info.name))
}

#[derive(Deserialize)]
struct FormData {
    username: String,
}

#[post("/form")]
async fn form(form: web::Form<FormData>) -> Result<String> {
    Ok(format!("Welcome {}", form.username))
}

#[derive(Clone)]
pub struct StateStruct {
    pub count: Cell<usize>,
}

#[get("/count")]
async fn show_count(data: web::Data<StateStruct>) -> impl Responder {
    format!("count: {}", data.count.get())
}

#[get("/add-one")]
async fn add_one(data: web::Data<StateStruct>) -> impl Responder {
    let count = data.count.get();
    data.count.set(count + 1);

    format!("Count: {}", data.count.get())
}

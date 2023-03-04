use actix_web::{get, post, web, http, body, error, Result, Error, Either, Responder, HttpRequest, HttpResponse};
use serde::{Serialize, Deserialize};
use futures::{future::ok, stream::once};
use actix_files::NamedFile;

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use std::cell::Cell;

// Application

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

// Server

#[get("/sleep")]
async fn sleep() -> impl Responder {
    tokio::time::sleep(Duration::from_secs(5)).await;
    "response"
}

#[get("/quit")]
async fn quit() -> HttpResponse {
    let mut res = HttpResponse::Ok()
        .force_close()
        .finish();

    res.head_mut().set_connection_type(http::ConnectionType::Close);
    res
}

// Extractors

#[derive(Deserialize)]
pub struct Extractors {
    pub id: u32,
    pub username: String,
}

#[derive(Deserialize)]
pub struct PostInfo {
    pub post_id: u32,
    pub friend: String,
}

#[derive(Deserialize)]
struct QueryStruct {
    name: String,
}

#[derive(Deserialize)]
struct JsonStruct {
    name: String,
}

#[derive(Deserialize)]
struct FormData {
    username: String,
}

#[derive(Clone)]
pub struct StateStruct {
    pub local_count: Cell<usize>,
    pub global_count: Arc<AtomicUsize>,
}

#[get("/extractors")]
async fn extractors(path: web::Path<(String, String)>, info: web::Json<Extractors>) -> impl Responder {
    let path = path.into_inner();
    format!("{} {} {} {}", path.0, path.1, info.id, info.username)
}

#[get("/posts/{post_id}/{friend}")]
async fn post_friend(req: HttpRequest) -> Result<String> {
    let name: String = req.match_info().get("friend").unwrap().parse().unwrap();
    let postid: i32 = req.match_info().query("post_id").parse().unwrap();

    Ok(format!("Welcome {}, post_id: {}", name, postid))
}

#[get("/query")]
async fn query(info: web::Query<QueryStruct>) -> String {
    format!("Welcome {}", info.name)
}

#[post("/json")]
async fn json(info: web::Json<JsonStruct>) -> Result<String> {
    Ok(format!("Welcome {}", info.name))
}

#[post("/form")]
async fn form(form: web::Form<FormData>) -> Result<String> {
    Ok(format!("Welcome {}", form.username))
}

#[get("/count")]
async fn show_count(data: web::Data<StateStruct>) -> impl Responder {
    format!("count: {}", data.local_count.get())
}

#[get("/add-one")]
async fn add_one(data: web::Data<StateStruct>) -> impl Responder {
    data.global_count.fetch_add(1, Ordering::Relaxed);

    let count = data.local_count.get();
    data.local_count.set(count + 1);

    format!("Count: {}", data.local_count.get())
}

// Handlers

#[derive(Serialize)]
struct CustomType {
    name: &'static str,
}

impl Responder for CustomType {
    type Body = body::BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
        .content_type(http::header::ContentType::json())
            .body(body)
    }
}

type RegisterResult = Either<HttpResponse, Result<&'static str, Error>>;

#[get("/responder")]
async fn responder(_req: HttpRequest) -> String {
    "Hello World!".to_owned()
}

#[get("/responder2")]
async fn responder_2(_req: HttpRequest) -> impl Responder {
    web::Bytes::from_static(b"Hello World!")
}

#[get("custom-type")]
async fn custom_type() -> impl Responder {
    CustomType { name: "ittokun" }
}

#[get("/stream")]
async fn stream() -> HttpResponse {
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"test")));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}

#[get("either")]
async fn either() -> RegisterResult {
    if true {
        Either::Left(HttpResponse::BadRequest().body("Bad data"))
    } else {
        Either::Right(Ok("Hello!"))
    }
}

// Errors

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display(fmt = "my error: {}", name)]
struct CustomError {
    name: &'static str,
}

impl error::ResponseError for CustomError {}

#[derive(Debug, derive_more::Display)]
enum CustomErrorEnum {
    #[display(fmt = "internal error")]
    InternalError,
    #[display(fmt = "bad request")]
    BadClientData,
    #[display(fmt = "timeout")]
    Timeout,
}

impl error::ResponseError for CustomErrorEnum {
    fn error_response(&self) -> HttpResponse<body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(http::header::ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> http::StatusCode {
        match *self {
            CustomErrorEnum::InternalError => http::StatusCode::INTERNAL_SERVER_ERROR,
            CustomErrorEnum::BadClientData => http::StatusCode::BAD_REQUEST,
            CustomErrorEnum::Timeout => http::StatusCode::GATEWAY_TIMEOUT,
        }
    }
}

#[get("/static-index")]
async fn static_index() -> std::io::Result<NamedFile> {
    Ok(NamedFile::open("static/index.html")?)
}

#[get("/custom-error")]
async fn custom_error() -> Result<&'static str, CustomError> {
    Err(CustomError { name: "test" })
}

#[get("/custom-error-enum")]
async fn custom_error_enum() -> Result<&'static str, CustomErrorEnum> {
    let internal_error = Err(CustomErrorEnum::InternalError)?;
    let _bad_client_data = Err(CustomErrorEnum::BadClientData)?;
    let _timeout = Err(CustomErrorEnum::Timeout)?;

    internal_error
}

#[get("/map-err")]
async fn map_err() -> Result<&'static str> {
    let result: Result<&'static str, CustomError> = Err(CustomError { name: "test error" });
    Ok(result.map_err(|e| error::ErrorBadRequest(e.name))?)
}

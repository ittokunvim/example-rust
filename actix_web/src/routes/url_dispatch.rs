use actix_web::{get, web, HttpResponse, guard};

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello")
}

#[get("/show")]
async fn show_users() -> HttpResponse {
    HttpResponse::Ok().body("Show users")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    // Resource configuration
    cfg.route("/url-dispatch", web::get().to(index));
    cfg.route("/url-dispatch/user", web::post().to(index));
    cfg.service(web::resource("/url-dispatch/prefix").to(index));
    cfg.service(
        web::resource("url-dispatch/user/{name}")
            .name("user_detail")
            .guard(guard::Header("content-type", "application/json"))
            .route(web::get().to(HttpResponse::Ok))
            .route(web::put().to(HttpResponse::Ok)),
    );
    // Configuring a Route
    cfg.service(
        web::resource("/url-dispatch/path").route(
            web::route()
                .guard(guard::Get())
                .guard(guard::Header("content-type", "text/plain"))
                .to(HttpResponse::Ok),
        ),
    );
}

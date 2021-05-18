use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;

pub mod config;
use crate::config::get_web_service_config;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/get-shortener-url")]
async fn hello_test() -> impl Responder {
    HttpResponse::Ok().body("Hello Test!!!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let (host, port) = get_web_service_config();

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(hello_test)
    })
    .bind(format!("{0}:{1}", host, port))?
    .run()
    .await
}

use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;

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
    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or("8080".to_string());

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(hello_test)
    })
    .bind(format!("{0}:{1}", host, port))?
    .run()
    .await
}

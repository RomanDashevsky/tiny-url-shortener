use actix_web::{App, HttpServer};
use dotenv::dotenv;

mod config;
mod controller;

use crate::config::get_web_service_config;
use crate::controller::{hello, hello_test};


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

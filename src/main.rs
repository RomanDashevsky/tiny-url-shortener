use actix_web::{App, HttpServer};
use dotenv::dotenv;
use mongodb::options::ClientOptions;
use mongodb::{Client, Collection};

mod config;
mod controller;
mod service;

use crate::config::{get_web_service_config, get_mongodb_config};
use crate::controller::{index, hello_test};
use crate::service::UrlService;

pub struct ServiceContainer {
    url: UrlService,
}

impl ServiceContainer {
    pub fn new(url: UrlService) -> Self {
        ServiceContainer { url }
    }
}

pub struct AppState {
    service_container: ServiceContainer,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let (host, port) = get_web_service_config();

    let url_collection = get_url_collection().await;

    HttpServer::new(move || {
        let service_container = ServiceContainer::new(UrlService::new(url_collection.clone()));
        App::new()
            .data(AppState { service_container })
            .service(index)
            .service(hello_test)
    })
    .bind(format!("{0}:{1}", host, port))?
    .run()
    .await
}

async fn get_url_collection() -> Collection {
    let (uri, db_name) = get_mongodb_config();
    let client_options = ClientOptions::parse(uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database(&db_name);
    db.collection("url")
}

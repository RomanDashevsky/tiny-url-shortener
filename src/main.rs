use actix_web::{App, HttpServer, HttpResponse, web};
use dotenv::dotenv;
use mongodb::options::ClientOptions;
use mongodb::{Client, Collection};

mod config;
mod controller;
mod service;

use crate::config::{get_web_service_config, get_mongodb_config};
use crate::controller::{find_url_and_redirect, insert_url};
use crate::service::{UrlService, Url};

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
            .route("/robots.txt", web::get().to(|| HttpResponse::NotFound()))
            .route("/favicon.ico", web::get().to(|| HttpResponse::NotFound()))
            .service(find_url_and_redirect)
            .service(insert_url)
    })
    .bind(format!("{0}:{1}", host, port))?
    .run()
    .await
}

async fn get_url_collection() -> Collection<Url> {
    let (uri, db_name) = get_mongodb_config();
    let client_options = ClientOptions::parse(&uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database(&db_name);
    db.collection_with_type::<Url>("url")
}

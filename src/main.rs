use actix_web::{App, HttpServer, HttpResponse, web, guard};
use dotenv::dotenv;
use mongodb::options::ClientOptions;
use mongodb::{Client, Collection};
use log::LevelFilter;
use log::{info};

mod config;
mod controller;
mod service;

use crate::config::{get_web_service_config, get_mongodb_config};
use crate::controller::{find_url_and_redirect, insert_url, delete_url};
use crate::service::{UrlService, Url};
use actix_web::dev::RequestHead;
use std::env;


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
    simple_logging::log_to_file("./server.log", LevelFilter::Info);

    let (host, port) = get_web_service_config();

    let url_collection = get_url_collection().await;

    info!("Server started with host: {0}, and port: {1}", host, port);

    HttpServer::new(move || {
        let service_container = ServiceContainer::new(UrlService::new(url_collection.clone()));
        App::new()
            .data(AppState { service_container })
            .route("/robots.txt", web::get().to(|| HttpResponse::NotFound()))
            .route("/favicon.ico", web::get().to(|| HttpResponse::NotFound()))
            .service(find_url_and_redirect)
            .service(
                web::scope("/api")
                    .guard(guard::fn_guard(is_auth))
                    .service(insert_url)
                    .service(delete_url)
            )
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

fn is_auth(head: &RequestHead) -> bool {
    let mut is_auth = false;
    let token_header = head.headers.get("X-API-TOKEN");

    let token = match token_header {
        Some(header) => {
            header.to_str().unwrap().to_string()
        },
        None => "".to_string()
    };

    if token == env::var("API_TOKEN").unwrap() {
        is_auth = true
    }

    is_auth
}
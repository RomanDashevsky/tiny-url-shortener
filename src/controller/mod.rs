use actix_web::{get, post, delete, Responder, HttpResponse, web, HttpRequest};
use crate::service::InsertUrlDto;
use std::env;

#[get("/{id}")]
pub async fn find_url_and_redirect(
    req: HttpRequest,
    app_data: web::Data<crate::AppState>
) -> impl Responder {
    let id: String = req.match_info().get("id").unwrap().parse().unwrap();
    let result = app_data.service_container.url.find_by_key(&id).await;
    match result {
        Ok(url_data) => {
            if url_data.is_none() {
                // TODO: add 404 html page
                return HttpResponse::NotFound().finish()
            }
            HttpResponse::Found().header("Location", url_data.unwrap().url).finish()
        }
        // TODO: add 404 html page
        Err(_) => HttpResponse::NotFound().finish()
    }
}

#[post("/api/url")]
pub async fn insert_url(req: HttpRequest, insert_url_dto: web::Json<InsertUrlDto>, app_data: web::Data<crate::AppState>) -> impl Responder {
    if !self::is_auth(&req) {
        // TODO: add 404 error response
        return HttpResponse::NotFound().finish()
    }

    let result = app_data.service_container.url.create(&insert_url_dto.url).await;
    match result {
        Ok(created_url_dto) => {
            if created_url_dto.is_none() {
                // TODO: add 404 error response
                return HttpResponse::NotFound().finish()
            }
            HttpResponse::Ok().json(created_url_dto.unwrap())
        },
        Err(_) => {
            // TODO: add 5xx error response
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[delete("/api/url")]
pub async fn delete_url(req: HttpRequest, insert_url_dto: web::Json<InsertUrlDto>, app_data: web::Data<crate::AppState>) -> impl Responder {
    if !self::is_auth(&req) {
        // TODO: add 404 error response
        return HttpResponse::NotFound().finish()
    }

    let result = app_data.service_container.url.delete(&insert_url_dto.url).await;
    match result {
        Ok(_) => {
            // TODO: add deleted response
            HttpResponse::Ok().finish()
        },
        Err(_) => {
            // TODO: add 5xx error response
            HttpResponse::InternalServerError().finish()
        }
    }
}

fn is_auth(req: &HttpRequest) -> bool {
    let mut is_auth = false;
    let token_header = req.headers().get("X-API-TOKEN");

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
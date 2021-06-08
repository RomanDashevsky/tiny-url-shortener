use actix_web::{get, post, delete, Responder, HttpResponse, web, HttpRequest};
use url::{Url};
use crate::service::InsertUrlDto;

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
        // TODO: add 5xx html page
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[post("/url")]
pub async fn insert_url(insert_url_dto: web::Json<InsertUrlDto>, app_data: web::Data<crate::AppState>) -> impl Responder {
    let url = &insert_url_dto.url;

    if !is_valid_url(url) {
        // TODO: add 400 error response
        return HttpResponse::BadRequest().finish()
    }

    let result = app_data.service_container.url.create(url).await;
    match result {
        Ok(created_url_dto) => {
            if created_url_dto.is_none() {
                // TODO: add 5xx error response
                HttpResponse::InternalServerError().finish()
            }
            HttpResponse::Ok().json(created_url_dto.unwrap())
        },
        Err(_) => {
            // TODO: add 5xx error response
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[delete("/url")]
pub async fn delete_url(insert_url_dto: web::Json<InsertUrlDto>, app_data: web::Data<crate::AppState>) -> impl Responder {
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

fn is_valid_url(url: &str) -> bool {
    let parsed_url_result = Url::parse(url);

    if parsed_url_result.is_err() {
        return false;
    }

    let parsed_url = parsed_url_result.unwrap();
    let scheme = parsed_url.scheme();
    parsed_url.has_host()
        && (scheme == "http"
        || scheme == "https")
}
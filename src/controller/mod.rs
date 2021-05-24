use actix_web::{get, post, Responder, HttpResponse, web, HttpRequest};
use crate::service::InsertUrlDto;

#[get("/{id}")]
pub async fn index(
    req: HttpRequest,
    app_data: web::Data<crate::AppState>
) -> impl Responder {
    let id: String = req.match_info().get("id").unwrap().parse().unwrap();
    let result = app_data.service_container.url.find_by_key(&id).await;
    match result {
        Ok(document) => {
            if document.is_none() {
                return HttpResponse::NotFound().finish()
            }
            HttpResponse::Ok().body(format!("Hello from index {0}", document.unwrap().url))
            // HttpResponse::TemporaryRedirect().header("Location", "/login").finish()
        },
        Err(_) => HttpResponse::NotFound().finish()
    }
}

#[post("/api/insert-url")]
pub async fn insert_url(insert_url_dto: web::Json<InsertUrlDto>, app_data: web::Data<crate::AppState>) -> impl Responder {
    let result = app_data.service_container.url.create(&insert_url_dto.url).await;
    match result {
        Ok(created_url_dto) => {
            if created_url_dto.is_none() {
                return HttpResponse::NotFound().finish()
            }
            HttpResponse::Ok().json(created_url_dto.unwrap())
        },
        Err(_) => {
            HttpResponse::InternalServerError().finish()
        }
    }

}

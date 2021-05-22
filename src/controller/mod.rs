use actix_web::{get, post, Responder, HttpResponse, web, HttpRequest};

#[get("/{id}")]
pub async fn index(
    req: HttpRequest,
    app_data: web::Data<crate::AppState>
) -> impl Responder {
    let id: String = req.match_info().get("id").unwrap().parse().unwrap();
    let result = app_data.service_container.url.create(&"/sadhgfh").await;
    HttpResponse::Ok().body(format!("Hello from index {0}, {1}", id, result.unwrap().inserted_id))
}

#[post("/get-shortener-url")]
pub async fn hello_test(app_data: web::Data<crate::AppState>) -> impl Responder {
    let result = app_data.service_container.url.get().await.unwrap();

    HttpResponse::Ok().body(format!("Hello Test!!! {0}", result.unwrap()))
}

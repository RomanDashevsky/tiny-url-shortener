use actix_web::{get, post, Responder, HttpResponse, web};

#[get("/{id}")]
pub async fn index(
    app_data: web::Data<crate::AppState>
) -> impl Responder {
    let result = app_data.service_container.url.create(&"/sadhgfh").await;
    HttpResponse::Ok().body(format!("Hello from index"))
}

#[post("/get-shortener-url")]
pub async fn hello_test(app_data: web::Data<crate::AppState>) -> impl Responder {
    let result = app_data.service_container.url.get().await.unwrap();

    HttpResponse::Ok().body(format!("Hello Test!!! {0}", result.unwrap()))
}

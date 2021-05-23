use actix_web::{get, post, Responder, HttpResponse, web, HttpRequest};

#[get("/{id}")]
pub async fn index(
    req: HttpRequest,
    app_data: web::Data<crate::AppState>
) -> impl Responder {
    let id: String = req.match_info().get("id").unwrap().parse().unwrap();
    let result = app_data.service_container.url.get(&id).await;
    match result {
        Ok(document) => {
            if document.is_none() {
                return HttpResponse::NotFound().finish()
            }
            HttpResponse::Ok().body(format!("Hello from index {0}", document.unwrap()))
            // HttpResponse::TemporaryRedirect().header("Location", "/login").finish()
        },
        Err(_) => HttpResponse::NotFound().finish()
    }
}

#[post("/get-shortener-url")]
pub async fn hello_test(app_data: web::Data<crate::AppState>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello Test!!!"))
}

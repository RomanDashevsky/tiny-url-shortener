use actix_web::{get, post, Responder, HttpResponse};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/get-shortener-url")]
pub async fn hello_test() -> impl Responder {
    HttpResponse::Ok().body("Hello Test!!!")
}

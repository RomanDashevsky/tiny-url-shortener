use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/get-shortener-url")]
async fn hello_test() -> impl Responder {
    HttpResponse::Ok().body("Hello Test!!!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(hello_test)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

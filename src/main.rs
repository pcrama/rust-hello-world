use actix_web::{web, App, HttpResponse, HttpServer};

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello, World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(web::resource("/hello-rust/").to(index))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}

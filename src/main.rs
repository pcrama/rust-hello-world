use hello_world_lib::create_app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_web::HttpServer::new(|| create_app())
        .bind("127.0.0.1:3000")?
        .run()
        .await
}

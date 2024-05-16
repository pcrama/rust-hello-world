use hello_world_lib::create_app;

fn configure_logging() {
    env_logger::Builder::from_env(env_logger::Env::default())
        .format_timestamp(None)
        .init();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    configure_logging();
    log::info!("Starting HttpServer...");
    actix_web::HttpServer::new(|| create_app())
        .bind("127.0.0.1:3000")?
        .run()
        .await
}

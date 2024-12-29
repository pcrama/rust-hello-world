use std::env;

use hello_world_lib::create_app;

fn configure_logging() {
    env_logger::Builder::from_env(env_logger::Env::default())
        .format_timestamp(None)
        .init();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    configure_logging();
    let bind_target = env::var("RUST_HELLO_WORLD_BIND_TO").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
    log::info!("Starting HttpServer...");
    actix_web::HttpServer::new(|| create_app())
        .bind(bind_target)?
        .run()
        .await
}

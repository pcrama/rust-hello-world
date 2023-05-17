use actix_web::{get, web, App, HttpResponse, HttpServer};

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello, World!")
}

/// extract path info from "/{user_id}/{name}" url
/// {user_id} - deserializes to a u32
/// {name} - deserializes to a String
#[get("/{user_id}/{name}")]
async fn greet_user_id_and_name(path: web::Path<(u32, String)>) -> HttpResponse {
    let (user_id, name) = path.into_inner();
    HttpResponse::Ok().body(format!("Welcome {}, user_id {}!", name, user_id))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(web::scope("/hello-rust")
	                       .service(greet_user_id_and_name)
			       .service(index))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}

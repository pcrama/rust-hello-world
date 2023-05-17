use actix_files::NamedFile;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer};
use std::path::PathBuf;
use tera::Tera;

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
    let tera = Tera::new("templates/**/*").unwrap();
    let mut context = tera::Context::new();
    context.insert("name", &name);
    context.insert("user_id", &user_id.to_string());
    let rendered = tera.render("greet_user_id_and_name.html", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/assets/{filename}")]
async fn static_files(req: HttpRequest) -> actix_web::Result<NamedFile> {
    let path: PathBuf = ["assets", req.match_info().query("filename")].iter().collect();
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(web::scope("/hello-rust")
			       .service(static_files)
	                       .service(greet_user_id_and_name)
			       .service(index))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}

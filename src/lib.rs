use actix_files::NamedFile;
use actix_web::{get, web, App, HttpRequest, HttpResponse};
use std::path::PathBuf;
use tera::Tera;

#[get("/")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello, World!")
}

#[get("/{user_id}/{name}")]
pub async fn greet_user_id_and_name(
    path: web::Path<(u32, String)>,
    tera: web::Data<Tera>,
) -> HttpResponse {
    let (user_id, name) = path.into_inner();
    let mut context = tera::Context::new();
    context.insert("name", &name);
    context.insert("user_id", &user_id.to_string());
    let rendered = tera
        .render("greet_user_id_and_name.html", &context)
        .unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/assets/{filename}")]
pub async fn static_files(req: HttpRequest) -> actix_web::Result<NamedFile> {
    let path: PathBuf = ["assets", req.match_info().query("filename")]
        .iter()
        .collect();
    Ok(NamedFile::open(path)?)
}

#[get("/forms/meter-readings")]
pub async fn get_meter_readings_form(tera: web::Data<Tera>) -> HttpResponse {
    let mut context = tera::Context::new();
    context.insert("timestamp", "2023-05-18 20:40"); // TODO: get current time & time zone correct

    // TODO: get SMA hostname & certificate from env & query its web interface with low timeout
    context.insert("pv_2022_prod_kWh", "1848");
    let rendered = tera.render("meter_readings_form.html", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}

// Type signature would have been impossible without
// https://github.com/actix/actix-web/issues/1147#issuecomment-1509937750.  See
// also its discussion of `configure' and
// https://github.com/actix/actix-web/issues/1402
pub fn create_app() -> App<
    impl actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse<impl actix_web::body::MessageBody>,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    App::new()
        .app_data(web::Data::new(Tera::new("templates/**/*").unwrap()))
        .service(
            web::scope("/hello-rust")
                .service(static_files)
                .service(get_meter_readings_form)
                .service(greet_user_id_and_name)
                .service(index),
        )
}

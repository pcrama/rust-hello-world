use actix_files::NamedFile;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse};
use serde::Deserialize;
use std::path::PathBuf;
use tera::Tera;

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct MeterReadings {
    timestamp: String,
    pv_2022_prod_kWh: u32,
    pv_2012_prod_kWh: u32,
    peak_hour_consumption_kWh: u32,
    off_hour_consumption_kWh: u32,
    peak_hour_injection_kWh: u32,
    off_hour_injection_kWh: u32,
    gas_m3: u32,
    water_m3: u32,
}

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

#[post("/meter-readings")]
pub async fn submit_meter_readings(form: web::Form<MeterReadings>) -> HttpResponse {
    // Perform the desired operations with the submitted data here
    log::info!(
        "Received data for {}: pv_2022_prod_kWh={}, pv_2012_prod_kWh={}, peak_hour_consumption_kWh={}, off_hour_consumption_kWh={}, peak_hour_injection_kWh={}, off_hour_injection_kWh={}, gas_m3={}, water_m3={}",
        form.timestamp,
	form.pv_2022_prod_kWh,
	form.pv_2012_prod_kWh,
	form.peak_hour_consumption_kWh,
	form.off_hour_consumption_kWh,
	form.peak_hour_injection_kWh,
	form.off_hour_injection_kWh,
	form.gas_m3,
	form.water_m3,);

    // Return a response indicating success or failure
    HttpResponse::Ok().body("Form submitted successfully")
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
                .service(submit_meter_readings)
                .service(greet_user_id_and_name)
                .service(index),
        )
}

use actix_files::NamedFile;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse};
use serde::Deserialize;
use std::path::PathBuf;
use tera::Tera;

pub fn empty_string_as_none(
    name: &str,
    opt_de: Option<&str>,
    errors: &mut Vec<String>,
) -> Option<f64> {
    match opt_de {
        None => None,
        Some(de) => {
            let opt = de.trim().replace(",", ".");
            if opt == "" {
                None
            } else {
                opt.parse().map(Some).unwrap_or_else(|e| {
                    errors.push(format!("{}: {}", name, e));
                    None
                })
            }
        }
    }
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct MeterReadingsUserInput {
    timestamp: String,
    pv_2022_prod_kWh: Option<String>,
    pv_2012_prod_kWh: Option<String>,
    peak_hour_consumption_kWh: Option<String>,
    off_hour_consumption_kWh: Option<String>,
    peak_hour_injection_kWh: Option<String>,
    off_hour_injection_kWh: Option<String>,
    gas_m3: Option<String>,
    water_m3: Option<String>,
}

#[allow(non_snake_case)]
struct MeterReadings {
    timestamp: String,
    pv_2022_prod_kWh: Option<f64>,
    pv_2012_prod_kWh: Option<f64>,
    peak_hour_consumption_kWh: Option<f64>,
    off_hour_consumption_kWh: Option<f64>,
    peak_hour_injection_kWh: Option<f64>,
    off_hour_injection_kWh: Option<f64>,
    gas_m3: Option<f64>,
    water_m3: Option<f64>,
}

fn parse_meter_values(ui: MeterReadingsUserInput) -> core::result::Result<MeterReadings, String> {
    let error_messages: &mut Vec<String> = &mut vec![];
    let result = MeterReadings {
        timestamp: ui.timestamp.clone(),
        pv_2012_prod_kWh: empty_string_as_none(
            "pv_2012_prod_kWh",
            ui.pv_2012_prod_kWh.as_deref(),
            error_messages,
        ),
        pv_2022_prod_kWh: empty_string_as_none(
            "pv_2022_prod_kWh",
            ui.pv_2022_prod_kWh.as_deref(),
            error_messages,
        ),
        peak_hour_consumption_kWh: empty_string_as_none(
            "peak_hour_consumption_kWh",
            ui.peak_hour_consumption_kWh.as_deref(),
            error_messages,
        ),
        off_hour_consumption_kWh: empty_string_as_none(
            "off_hour_consumption_kWh",
            ui.off_hour_consumption_kWh.as_deref(),
            error_messages,
        ),
        peak_hour_injection_kWh: empty_string_as_none(
            "peak_hour_injection_kWh",
            ui.peak_hour_injection_kWh.as_deref(),
            error_messages,
        ),
        off_hour_injection_kWh: empty_string_as_none(
            "off_hour_injection_kWh",
            ui.off_hour_injection_kWh.as_deref(),
            error_messages,
        ),
        gas_m3: empty_string_as_none("gas_m3", ui.gas_m3.as_deref(), error_messages),
        water_m3: empty_string_as_none("water_m3", ui.water_m3.as_deref(), error_messages),
    };

    if error_messages.len() == 0 {
        Ok(result)
    } else {
        Err(error_messages.join("; "))
    }
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
pub async fn submit_meter_readings(
    web::Form(form): web::Form<MeterReadingsUserInput>,
) -> HttpResponse {
    match parse_meter_values(form) {
        Ok(mr) => {
            // Perform the desired operations with the submitted data here
            let msg = format!(
                "Received data for {}: pv_2022_prod_kWh={}, pv_2012_prod_kWh={}, peak_hour_consumption_kWh={}, off_hour_consumption_kWh={}, peak_hour_injection_kWh={}, off_hour_injection_kWh={}, gas_m3={}, water_m3={}",
                mr.timestamp,
		mr.pv_2022_prod_kWh.unwrap_or(-99.9),
		mr.pv_2012_prod_kWh.unwrap_or(-99.9),
		mr.peak_hour_consumption_kWh.unwrap_or(-99.9),
		mr.off_hour_consumption_kWh.unwrap_or(-99.9),
		mr.peak_hour_injection_kWh.unwrap_or(-99.9),
		mr.off_hour_injection_kWh.unwrap_or(-99.9),
		mr.gas_m3.unwrap_or(-99.9),
		mr.water_m3.unwrap_or(-99.9),
	    );
            log::info!("{}", msg);
            HttpResponse::Ok().body(format!("Form submitted successfully: {}", msg))
        }
        Err(s) => {
            log::error!("Unable to parse inputs: {}", s);
            HttpResponse::Ok().body(format!("Bad input data: {}", s))
        }
    }
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

use actix_files::NamedFile;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use tokio::process::Command;
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

#[derive(Deserialize,Serialize)]
#[allow(non_snake_case)]
pub struct MeterReadingsUserInput {
    pub timestamp: String,
    pub pv_2022_prod_kWh: Option<String>,
    pub pv_2012_prod_kWh: Option<String>,
    pub peak_hour_consumption_kWh: Option<String>,
    pub off_hour_consumption_kWh: Option<String>,
    pub peak_hour_injection_kWh: Option<String>,
    pub off_hour_injection_kWh: Option<String>,
    pub gas_m3: Option<String>,
    pub water_m3: Option<String>,
}

impl ToString for MeterReadingsUserInput {
    #[allow(non_snake_case)]
    fn to_string(&self) -> String {
        let timestamp = self.timestamp.to_string();
        let pv_2022_prod_kWh = self.pv_2022_prod_kWh.as_deref().unwrap_or("null");
        let pv_2012_prod_kWh = self.pv_2012_prod_kWh.as_deref().unwrap_or("null");
        let peak_hour_consumption_kWh = self.peak_hour_consumption_kWh.as_deref().unwrap_or("null");
        let off_hour_consumption_kWh = self.off_hour_consumption_kWh.as_deref().unwrap_or("null");
        let peak_hour_injection_kWh = self.peak_hour_injection_kWh.as_deref().unwrap_or("null");
        let off_hour_injection_kWh = self.off_hour_injection_kWh.as_deref().unwrap_or("null");
        let gas_m3 = self.gas_m3.as_deref().unwrap_or("null");
        let water_m3 = self.water_m3.as_deref().unwrap_or("null");

        format!(
            "MeterReadingsUserInput(timestamp={}, pv_2022_prod_kWh={}, pv_2012_prod_kWh={}, peak_hour_consumption_kWh={}, off_hour_consumption_kWh={}, peak_hour_injection_kWh={}, off_hour_injection_kWh={}, gas_m3={}, water_m3={})",
            timestamp,
            pv_2022_prod_kWh,
            pv_2012_prod_kWh,
            peak_hour_consumption_kWh,
            off_hour_consumption_kWh,
            peak_hour_injection_kWh,
            off_hour_injection_kWh,
            gas_m3,
            water_m3
        )
    }
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

fn get_env_var(name: &str) -> core::result::Result<String, String> {
    return std::env::var(name).map_err(|_| format!("Set up '{}' with a value.", name));
}

pub fn get_ip_address(dhcp_lease_file: &str, hostname: &str) -> String {
    let file = match File::open(dhcp_lease_file) {
        Ok(f) => f,
        Err(_) => return hostname.to_string(),
    };
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => return hostname.to_string(),
        };
        let columns: Vec<&str> = line.split_whitespace().collect();

        if columns.len() >= 4 && columns[3].eq_ignore_ascii_case(hostname) {
            return columns[2].to_string()
        }
    }

    return hostname.to_string();
}

async fn fetch_dashboard_value() -> core::result::Result<f64, String> {
    let server_host = get_env_var("RUST_HELLO_WORLD_REMOTE_SERVER_HOST")
        .map_err(|e| format!("Missing environment variable: {}", e))?;
    let server_path = get_env_var("RUST_HELLO_WORLD_REMOTE_SERVER_PATH")
        .map_err(|e| format!("Missing environment variable: {}", e))?;
    let dashboard_url = format!("https://{}/{}", server_host, server_path);

    // Call the external curl command asynchronously
    let response = Command::new("curl")
        .arg("--silent")  // Silent mode, suppresses progress bar
        .arg("--insecure")  // don't worry about certificates
        .arg("--connect-timeout").arg("1")
        .arg("--max-time").arg("2")
        .arg(dashboard_url)
        .output()
        .await
        .map_err(|e| format!("Failed to execute curl: {}", e))?;

    if !response.status.success() {
        return Err(format!(
            "Curl request failed with status: {}... {}",
            response.status,
            std::str::from_utf8(&response.stderr).map_err(|e| format!("Failed to decode curl's stderr: {}", e))?
        ));
    }

    // Convert the output to a UTF-8 string
    let response_text = std::str::from_utf8(&response.stdout)
        .map_err(|e| format!("Failed to parse curl response as UTF-8: {}", e))?;

    print!("response_text={}", response_text);
    let json: Value = serde_json::from_str(response_text)
        .map_err(|e| format!("Unable to parse JSON: {}", e))?;
    let value = json["result"]["0199-xxxxx9BD"]["6400_00260100"]["1"][0]["val"]
        .as_f64()
        .ok_or("Invalid JSON response")?;

    Ok(value)
}

#[get("/forms/meter-readings")]
pub async fn get_meter_readings_form(tera: web::Data<Tera>) -> HttpResponse {
    let mut context = tera::Context::new();
    context.insert("timestamp", "2023-05-18 20:40"); // TODO: get current time & time zone correct

    match fetch_dashboard_value().await {
        Ok(value) => {
            context.insert("pv_2022_prod_kWh", &(value / 1000.0).to_string());
        }
        Err(err) => {
            log::error!("Failed to fetch dashboard value: {}", err);
            let error_message: String = if err.to_string().contains("Request timed out") {
                "Timeout".to_string()
            } else {
                format!("Error fetching value: {}", err)
            };
            context.insert("pv_2022_prod_kWh", error_message.as_str());
        }
    }

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

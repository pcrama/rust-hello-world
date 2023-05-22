use actix_web::{http::StatusCode, test};

use hello_world_lib::create_app;

#[actix_rt::test]
async fn test_greet_user_id_and_name() {
    let mut app = test::init_service(create_app()).await;

    let user_id = 42;
    let name = "John".to_string();
    let request = test::TestRequest::get()
        .uri(&format!("/hello-rust/{}/{}", user_id, name))
        .to_request();

    let response = test::call_service(&mut app, request).await;
    assert_eq!(response.status(), StatusCode::OK);

    let body = test::read_body(response).await;
    let body_str = String::from_utf8_lossy(&body).to_string();

    let expected_response = format!("<!DOCTYPE html>\n<html>\n  <head>\n    <title>Greeting</title>\n    <link rel=\"apple-touch-icon\" sizes=\"180x180\" href=\"/hello-rust/assets/apple-touch-icon.png\">\n    <link rel=\"icon\" type=\"image/png\" sizes=\"32x32\" href=\"/hello-rust/assets/favicon-32x32.png\">\n    <link rel=\"icon\" type=\"image/png\" sizes=\"16x16\" href=\"/hello-rust/assets/favicon-16x16.png\">\n    <link rel=\"manifest\" href=\"/hello-rust/assets/site.webmanifest\">\n  </head>\n  <body>\n    <h1>Hello, {}!</h1>\n    <p>Your user ID is {}.</p>\n  </body>\n</html>\n", name, user_id);
    assert_eq!(body_str, expected_response);
}

#[actix_rt::test]
async fn test_get_meter_readings_form() {
    let mut app = test::init_service(create_app()).await;

    let request = test::TestRequest::get()
        .uri("/hello-rust/forms/meter-readings")
        .to_request();

    let response = test::call_service(&mut app, request).await;
    assert_eq!(response.status(), StatusCode::OK);

    let body = test::read_body(response).await;
    let body_str = String::from_utf8_lossy(&body).to_string();

    for id in [
        "pv_2012_prod_kWh",
        "pv_2022_prod_kWh",
        "peak_hour_consumption_kWh",
        "off_hour_consumption_kWh",
        "peak_hour_injection_kWh",
        "off_hour_injection_kWh",
        "gas_m3",
        "water_m3",
    ]
    .iter()
    {
        let expected_label = format!("<label for=\"{}\">", id);
        assert!(
            body_str.contains(&expected_label),
            "Response does not contain expected HTML: label",
        );

        let expected_input = format!(
            "<input type=\"text\" pattern=\"^ *(|\\d+([.,]\\d)?) *$\" id=\"{}\" name=\"{}\"",
            id, id
        );
        assert!(
            body_str.contains(&expected_input),
            "Response does not contain expected HTML: input",
        );
    }
}

use actix_web::{http::StatusCode, test};
use hello_world_lib::{create_app, MeterReadingsUserInput};

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

#[actix_rt::test]
async fn test_submit_meter_readings() {
    let mut app = test::init_service(create_app()).await;

    // Create a mock form input
    let form_input = MeterReadingsUserInput {
        timestamp: "2023-05-22 20:40".to_string(),
        pv_2022_prod_kWh: Some("1848.2".to_string()),
        pv_2012_prod_kWh: None,
        peak_hour_consumption_kWh: Some(" 10.5  ".to_string()),
        off_hour_consumption_kWh: Some(" ".to_string()),
        peak_hour_injection_kWh: None,
        off_hour_injection_kWh: None,
        gas_m3: Some("  5,6".to_string()),
        water_m3: Some("6,5  ".to_string()),
    };

    // Encode the form input into the body content
    let form_body = form_input.to_string();

    // Create a test request and send it to the server
    let req = test::TestRequest::post()
        .uri("/hello-rust/meter-readings")
        .set_form(form_body)
        .to_request();
    let resp = test::call_service(&mut app, req).await;

    // Assert that the response is successful
    assert!(resp.status().is_success());

    // Read the response body
    let body = test::read_body(resp).await;

    // Assert that the response body contains the expected message
    let expected_msg = "Form submitted successfully: Received data for 2023-05-22 20:40: pv_2022_prod_kWh=1848.2, pv_2012_prod_kWh=-99.9, peak_hour_consumption_kWh=10.5, off_hour_consumption_kWh=-99.9, peak_hour_injection_kWh=-99.9, off_hour_injection_kWh=-99.9, gas_m3=5.6, water_m3=6.5";
    assert_eq!(body, expected_msg.as_bytes());
}

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

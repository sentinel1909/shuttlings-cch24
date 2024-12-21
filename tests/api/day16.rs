// tests/api/day16.rs

// dependencies
use crate::helpers::spawn_app;
use jsonwebtoken::{encode, Header, EncodingKey};

#[tokio::test]
async fn day16_wrap_endpoint_accepts_arbitrary_json_and_returns_200_ok() {
    // Arrange
    let app = spawn_app().await;
    let request_body = "{\"cookie is delicious\":\"true\"}";
    
    // Act
    let response = app
        .application_client
        .post(format!("{}/16/wrap", &app.application_address))
        .json(&request_body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
}

#[tokio::test]
async fn day16_wrap_endpoint_sets_cookie_header_with_jwt_and_returns_200_ok() {
    // Arrange
    let app = spawn_app().await;
    let request_body = "{\"cookie is delicious\":\"true\"}";

    // Act
    let response = app
        .application_client
        .post(format!("{}/16/wrap", &app.application_address))
        .json(&request_body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    let response_header = response.headers().get("SET-COOKIE").unwrap();
    let jwt = encode(&Header::default(), &request_body, &EncodingKey::from_secret(b"the_secret_key")).unwrap();
    let expected_header =  format!("gift={}", jwt);
    assert_eq!(response_header.to_owned(), expected_header);
}

// tests/api/day16.rs

// dependencies
use crate::helpers::spawn_app;
use axum::http::StatusCode;
use serde_json::json;

#[tokio::test]
async fn wrap_endpoint_receives_arbitrary_json_request_body_and_returns_200_ok() {
    // Arrange
    let app = spawn_app().await;
    let request_body = json!({"contents": "toy airplane"});

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
async fn wrap_endpoint_rejects_malformed_json_request_body_and_returns_400_bad_request() {
    // Arrange
    let app = spawn_app().await;
    let request_body = json!({"bad json": "so bad"});

    // Act
    let response = app
        .application_client
        .post(format!("{}/16/wrap", &app.application_address))
        .json(&request_body)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn wrap_endpoint_contains_the_set_cookie_header_and_a_200_ok_response() {
    // Arrange
    let app = spawn_app().await;
    let request_body = json!({"contents":"battleship model"});

    // Act
    let response = app
        .application_client
        .post(format!("{}/16/wrap", &app.application_address))
        .json(&request_body)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    let response_header = response.headers().get("Set-Cookie").unwrap();
    let expected_header = "gift=battleship model";
    assert_eq!(response_header.to_str().unwrap(), expected_header);
}

#[tokio::test]
async fn unwrap_endpoint_responds_with_json_and_200_ok() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app
        .application_client
        .get(format!("{}/16/unwrap", &app.application_address))
        .header("Set-Cookie", "gift={JWT}")
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
}

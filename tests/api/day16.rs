// tests/api/day16.rs

// dependencies
use crate::helpers::spawn_app;
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

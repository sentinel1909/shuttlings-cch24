// tests/api/day19.rs

// dependencies
use crate::helpers::spawn_app;
use axum::http::StatusCode;
use serde_json::json;

#[tokio::test]
async fn draft_endpoint_adds_quote_with_uuid_and_returns_the_quote_with_201_created() {
    // Arrange
    let app = spawn_app().await;
    let request_body = json!({
      "author":"Santa",
      "quote":"Ho ho ho!"
    });

    // Act
    let response = app
        .application_client
        .post(format!("{}/19/draft", &app.application_address))
        .json(&request_body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), StatusCode::CREATED);
}

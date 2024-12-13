// tests/api/day9.rs

// dependences
use crate::helpers::spawn_app;
use axum::http::StatusCode;

#[tokio::test]
async fn day_9_task1_responds_with_200_ok_if_milk_is_available() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app
        .application_client
        .post(format!("{}/9/milk", &app.application_address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
}

#[tokio::test]
async fn day_9_task1_returns_429_too_many_requests_if_bucket_empty() {
    // Arrange
    let app = spawn_app().await;

    {
        let rate_limiter = app.application_state.rate_limiter.lock().await;
        while rate_limiter.try_acquire(1) {}
    }

    // Act
    let response = app
        .application_client
        .post(format!("{}/9/milk", &app.application_address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
    let response_body = response
        .text()
        .await
        .expect("Unable to retrieve response body.");
    let expected_body = "No milk available\n";
    assert_eq!(response_body, expected_body);
}

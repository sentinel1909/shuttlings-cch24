// tests/api/day_minus_one.rs

// dependencies
use crate::helpers::spawn_app;
use http::{header, StatusCode};

#[tokio::test]
async fn day_minus1_task1_endpoint_works() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app
        .application_client
        .get(format!("{}/", &app.application_address))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    let response_body = response
        .text()
        .await
        .expect("Unable to retrieve the response body.");
    let expected_body = "Hello, bird!".to_string();
    assert_eq!(response_body, expected_body);
}

#[tokio::test]
async fn day_minus1_task2_endpoint_works() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app
        .application_client
        .get(format!("{}/-1/seek", &app.application_address))
        .send()
        .await
        .expect("Failed to execute request.");
    println!("{:?}", response);

    // Assert
    assert_eq!(response.status(), StatusCode::FOUND);
    let location_header = response.headers().get(header::LOCATION).unwrap();
    assert_eq!(
        location_header,
        "https://www.youtube.com/watch?v=9Gc4QTqslN4"
    );
}

// tests/api/day2.rs

// dependencies
use crate::helpers::spawn_app;

#[tokio::test]
async fn day2_task1_endpoint_works() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app
        .application_client
        .get(format!("{}/2/dest", &app.application_address))
        .query(&[("from", "10.0.0.0"), ("key", "1.2.3.255")])
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    let response_body = response
        .text()
        .await
        .expect("Unable to retrieve response body.");
    let expected_body = "11.2.3.255";
    assert_eq!(response_body, expected_body);
}
#[tokio::test]
async fn day2_task2_endpoint_works() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app
        .application_client
        .get(format!("{}/2/key", &app.application_address))
        .query(&[("from", "10.0.0.0"), ("to", "11.2.3.255")])
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    let response_body = response
        .text()
        .await
        .expect("Unable to retrieve response body.");
    let expected_body = "1.2.3.255";
    assert_eq!(response_body, expected_body);
}

// tests/api/day23.rs

// dependencies
use crate::helpers::spawn_app;

#[tokio::test]
async fn day23_task2_endpoint_returns_html_to_light_star_and_200_ok() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app
        .application_client
        .get(format!("{}/23/star", &app.application_address))
        .send()
        .await
        .expect("Failed to send request.");

    // Assert
    assert!(response.status().is_success());
    let response_body = response
        .text()
        .await
        .expect("Unable to retrieve response body.");
    let expected_body = "<div id=\"star\" class=\"lit\"></div>";
    assert_eq!(response_body, expected_body);
}

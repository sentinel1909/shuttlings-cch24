// test/api/day5.rs

use http::{header, StatusCode};

// dependencies
use crate::helpers::spawn_app;

#[tokio::test]
async fn day5_task1_endpoint_works_for_valid_data() {
    // Arrange
    let app = spawn_app().await;
    let test_manifest = r#"
  [package]
  name = "not-a-gift-order"
  authors = ["Not Santa"]
  keywords = ["Christmas 2024"]

  [[package.metadata.orders]]
  item = "Toy car"
  quantity = 2

  [[package.metadata.orders]]
  item = "Lego brick"
  quantity = 230
  "#;

    // Act
    let response = app
        .application_client
        .post(format!("{}/5/manifest", &app.application_address))
        .header(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/toml"),
        )
        .body(test_manifest)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    let response_body = response
        .text()
        .await
        .expect("Unable to retrieve the response body.");
    let expected_body = "Toy car: 2\nLego brick: 230";
    assert_eq!(response_body, expected_body);
}

#[tokio::test]
async fn day5_task1_endpoint_responds_with_204_for_no_valid_data() {
    // Arrange
    let app = spawn_app().await;
    let test_manifest = r#"
  [package]
  name = "coal-in-a-bowl"
  authors = ["H4CK3R_13E7"]
  keywords = ["Christmas 2024"]

  [[package.metadata.orders]]
  item = "Coal"
  quantity = "Haha get rekt"

  "#;

    // Act
    let response = app
        .application_client
        .post(format!("{}/5/manifest", &app.application_address))
        .header(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/toml"),
        )
        .body(test_manifest)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn day5_task1_endpoint_responds_with_204_for_empty_data() {
    // Arrange
    let app = spawn_app().await;
    let test_manifest = r#"
[package]
name = "coal-in-a-bowl"
authors = ["H4CK3R_13E7"]
keywords = ["Christmas 2024"]

package.metadata.orders = []
"#;

    // Act
    let response = app
        .application_client
        .post(format!("{}/5/manifest", &app.application_address))
        .header(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/toml"),
        )
        .body(test_manifest)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn day5_task1_endpoint_ignores_invalid_data() {
    // Arrange
    let app = spawn_app().await;
    let test_manifest = r#"
[package]
name = "not-a-gift-order"
authors = ["Not Santa"]
keywords = ["Christmas 2024"]

[[package.metadata.orders]]
item = "Toy car"
quantity = 2

[[package.metadata.orders]]
item = "Lego brick"
quantity = 1.5

[[package.metadata.orders]]
item = "Doll"
quantity = 2

[[package.metadata.orders]]
quantity = 5
item = "Cookie:::\n"

[[package.metadata.orders]]
item = "Thing"
count = 3
"#;

    // Act
    let response = app
        .application_client
        .post(format!("{}/5/manifest", &app.application_address))
        .header(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/toml"),
        )
        .body(test_manifest)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), StatusCode::OK);
}

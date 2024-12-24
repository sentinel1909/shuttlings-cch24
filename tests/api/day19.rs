// tests/api/day19.rs

// dependencies
use crate::helpers::spawn_app;
use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

// struct type to represent the ApiResponse
#[derive(Deserialize, Debug)]
struct ApiResponse {
    id: String,
    author: String,
    quote: String,
    created_at: String,
    version: i32,
}

#[tokio::test]
async fn day19_draft_endpoint_adds_quote_with_uuid_and_returns_the_quote_with_201_created() {
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

    // Assert the response returns 201 CREATED
    assert_eq!(response.status(), StatusCode::CREATED);
    let response_body = response.bytes().await.unwrap();
    let response_data: ApiResponse = serde_json::from_slice(&response_body).unwrap();

    // Assert the static fields
    assert_eq!(response_data.author, "Santa");
    assert_eq!(response_data.quote, "Ho ho ho!");
    assert_eq!(response_data.version, 1);

    // Assert the id field has the correct form
    let id = Uuid::parse_str(&response_data.id);
    assert!(id.is_ok(), "Invalid UUID forid field.");

    // Assert the created_at field has the correct form
    let created_at = response_data.created_at.parse::<DateTime<Utc>>();
    assert!(
        created_at.is_ok(),
        "Invalid timestamp for created_at field."
    );
}

#[tokio::test]
async fn day19_undo_endpoint_updates_author_and_quote_returns_the_updated_quote_with_200_ok() {
    // Arrange
    let app = spawn_app().await;
    let request_body = json!({
      "author":"Santa",
      "quote":"Ho ho ho!"
    });

    let undo_body = json!({
      "author":"Santa",
      "quote":"I changed my mind..."
    });

    // Act - Part 1
    let response = app
        .application_client
        .post(format!("{}/19/draft", &app.application_address))
        .json(&request_body)
        .send()
        .await
        .expect("Failed to execute request.");

    let response_body = response.bytes().await.unwrap();
    let response_data: ApiResponse = serde_json::from_slice(&response_body).unwrap();
    let id = response_data.id;

    // Act - Part 2
    let undo_response = app
        .application_client
        .put(format!("{}/19/undo/{}", &app.application_address, id))
        .json(&undo_body)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert - updated response
    assert!(undo_response.status().is_success());
    let undo_response_body = undo_response.bytes().await.unwrap();
    let undo_response_data: ApiResponse = serde_json::from_slice(&undo_response_body).unwrap();

    // Assert the static fields for the updated response
    assert_eq!(undo_response_data.author, "Santa");
    assert_eq!(undo_response_data.quote, "I changed my mind...");
    assert_eq!(undo_response_data.version, 2);

    // Assert the id field has the correct form in the updated response
    let id = Uuid::parse_str(&undo_response_data.id);
    assert!(id.is_ok(), "Invalid UUID forid field.");

    // Assert the created_at field has the correct form in the updated response
    let created_at = response_data.created_at.parse::<DateTime<Utc>>();
    assert!(
        created_at.is_ok(),
        "Invalid timestamp for created_at field."
    );
}

#[tokio::test]
async fn day19_reset_endpoint_clears_the_quotes_table_and_returns_200_ok() {
    // Arrange
    let app = spawn_app().await;
    let request_body = json!({
      "author":"Santa",
      "quote":"Ho ho ho!"
    });

    // Act - Part 1
    let _response = app
        .application_client
        .post(format!("{}/19/draft", &app.application_address))
        .json(&request_body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Act - Part 2
    let reset_response = app
        .application_client
        .post(format!("{}/19/reset", &app.application_address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(reset_response.status().is_success());
    let reset_response_body = reset_response
        .text()
        .await
        .expect("Failed to read response body.");
    assert!(reset_response_body.is_empty());
}

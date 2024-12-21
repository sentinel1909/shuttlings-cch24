// tests/api/day12.rs

// dependencies
use crate::helpers::spawn_app;

#[tokio::test]
pub async fn day12_get_board_state_responds_with_current_board_and_200_ok() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app
        .application_client
        .get(format!("{}/12/board", &app.application_address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    let response_body = response
        .text()
        .await
        .expect("Unable to retrieve response body.");
    let expected_body = "⬜⬛⬛⬛⬛⬜\n⬜⬛⬛⬛⬛⬜\n⬜⬛⬛⬛⬛⬜\n⬜⬛⬛⬛⬛⬜\n⬜⬜⬜⬜⬜⬜\n";
    assert_eq!(response_body, expected_body);
}

#[tokio::test]
pub async fn day12_reset_board_state_responds_with_empty_board_and_200_ok() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app
        .application_client
        .post(format!("{}/12/reset", &app.application_address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    let response_body = response
        .text()
        .await
        .expect("Unable to retrieve response body.");
    let expected_body = "⬜⬛⬛⬛⬛⬜\n⬜⬛⬛⬛⬛⬜\n⬜⬛⬛⬛⬛⬜\n⬜⬛⬛⬛⬛⬜\n⬜⬜⬜⬜⬜⬜\n";
    assert_eq!(response_body, expected_body);
}

#[tokio::test]
pub async fn day12_post_place_item_responds_with_board_state_and_200_ok() {
    // Arrange
    let app = spawn_app().await;
    let team = "cookie";
    let column = 1;

    // Act
    let response = app
        .application_client
        .post(format!(
            "{}/12/place/{}/{}",
            &app.application_address, &team, &column
        ))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    let response_body = response
        .text()
        .await
        .expect("Unable to retrieve response body");
    let expected_body = "⬜⬛⬛⬛⬛⬜\n⬜⬛⬛⬛⬛⬜\n⬜⬛⬛⬛⬛⬜\n⬜🍪⬛⬛⬛⬜\n⬜⬜⬜⬜⬜⬜\n";
    assert_eq!(response_body, expected_body);
}

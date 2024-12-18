// tests/api/day12.rs

// dependencies
use crate::helpers::spawn_app;
use axum::http::StatusCode;

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
pub async fn day12_post_play_game_place_cookie_column_1_responds_with_board_state_and_200_ok() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app
        .application_client
        .post(format!(
            "{}/12/place/{}/{}",
            &app.application_address, "cookie", 1
        ))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    let response_body = response
        .text()
        .await
        .expect("Unable to retrieve response body.");
    let expected_body = "⬜⬛⬛⬛⬛⬜\n⬜⬛⬛⬛⬛⬜\n⬜⬛⬛⬛⬛⬜\n⬜🍪⬛⬛⬛⬜\n⬜⬜⬜⬜⬜⬜\n";
    assert_eq!(response_body, expected_body);
}

#[tokio::test]
pub async fn day12_post_play_game_returns_board_state_and_503_service_unavailable_if_straight_row_winner(
) {
    // Arrange
    let app = spawn_app().await;

    // Act
    let moves: [i32; 4] = [1, 2, 3, 4];
    let response: Option<reqwest::Response> = None;

    for _i in moves.iter() {
        let _response = Some(
            app.application_client
                .post(format!(
                    "{}/12/place/{}/{}",
                    &app.application_address, "cookie", 1
                ))
                .send()
                .await
                .expect("Failed to execute request."),
        );
    }

    // Assert
    if let Some(resp) = response {
        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
        let response_body = resp
            .text()
            .await
            .expect("Unable to retrieve response body.");
        let expected_body =
            "⬜⬛⬛⬛⬛⬜\n⬜⬛⬛⬛⬛⬜\n⬜⬛⬛⬛⬛⬜\n⬜🍪🍪🍪🍪⬜\n⬜⬜⬜⬜⬜⬜\n🍪 wins!\n";
        assert_eq!(response_body, expected_body);
    }
}

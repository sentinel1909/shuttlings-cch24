// src/lib/routes/day9.rs

// dependencies
use crate::AppState;
use axum::{
    body::Body,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_macros::debug_handler;

// Day 9, Task 1 handler
#[debug_handler]
#[tracing::instrument(name = "Day 9, Task 1 Handler", skip(state))]
pub async fn day9_task1(State(state): State<AppState>) -> impl IntoResponse {
    let rate_limiter = state.rate_limiter.lock().await;
    let milk = rate_limiter.try_acquire(1);
    if milk {
        "Milk withdrawn\n".into_response()
    } else {
        Response::builder()
            .status(StatusCode::TOO_MANY_REQUESTS)
            .body(Body::from("No milk available\n"))
            .unwrap()
            .into_response()
    }
}

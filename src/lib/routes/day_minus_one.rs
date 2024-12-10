// src/lib/routes/minus_one.rs

// dependencies
use axum::{
    body::Body,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use axum_macros::debug_handler;

// Day -1, Task 1 handler
#[debug_handler]
#[tracing::instrument(name = "Day Minus 1, Task 1")]
pub async fn day_minus_one_task1() -> impl IntoResponse {
    "Hello, bird!".to_string()
}

// Day -1, Task 2 handler
#[debug_handler]
#[tracing::instrument(name = "Day Minus 1, Task 2")]
pub async fn day_minus_one_task2() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::FOUND)
        .header(
            header::LOCATION,
            "https://www.youtube.com/watch?v=9Gc4QTqslN4",
        )
        .body(Body::empty())
        .unwrap()
}

// src/main.rs

// dependencies
use axum::{
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};

// Day -1, Task 1 handler
async fn dayminus1_task1() -> impl IntoResponse {
    "Hello, bird!"
}

// Day -1, Task 2 handler
async fn dayminus1_task2() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::LOCATION,
        "https://www.youtube.com/watch?v=9Gc4QTqslN4"
            .parse()
            .unwrap(),
    );
    (headers, StatusCode::FOUND)
}

// main function
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(dayminus1_task1))
        .route("/-1/seek", get(dayminus1_task2));

    Ok(router.into())
}

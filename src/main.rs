// src/main.rs

// dependencies
use axum::{response::IntoResponse, routing::get, Router};

// Day -1, Task 1 handler
async fn dayminus1_task1() -> impl IntoResponse {
    "Hello, bird!"
}

// main function
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(dayminus1_task1));

    Ok(router.into())
}

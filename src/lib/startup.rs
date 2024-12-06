// src/lib/startup.rs

// dependencies
use crate::routes::handlers::{day2_task1, day2_task2, dayminus1_task1, dayminus1_task2};
use axum::{routing::get, Router};

// function to build and return a router type, configured with all the necessary routes and handlers
pub fn build_router() -> Router {
    Router::new()
        .route("/", get(dayminus1_task1))
        .route("/-1/seek", get(dayminus1_task2))
        .route("/2/dest", get(day2_task1))
        .route("/2/key", get(day2_task2))
}

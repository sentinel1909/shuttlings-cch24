// src/lib/routes/day16.rs

// dependencies
use axum::{
    extract::rejection::JsonRejection, extract::Json, http::StatusCode, response::IntoResponse,
};
use axum_macros::debug_handler;
use serde::Deserialize;

// struct type to represent the incoming gift, represented as JSON
#[derive(Debug, Deserialize)]
pub struct Gift {
    pub contents: String,
}

// Day 16 Task 1 handler
#[debug_handler]
#[tracing::instrument(name = "Day 16, Task 1 Handler")]
pub async fn day16_task1(payload: Result<Json<Gift>, JsonRejection>) -> impl IntoResponse {
    match payload {
        Ok(_) => return StatusCode::OK,
        Err(_) => return StatusCode::BAD_REQUEST,
    }
}

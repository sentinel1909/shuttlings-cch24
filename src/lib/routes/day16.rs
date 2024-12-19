// src/lib/routes/day16.rs

// dependencies
use axum::{
    extract::{rejection::JsonRejection, Json},
    http::StatusCode,
    response::IntoResponse
};
use axum_macros::debug_handler;
use serde::Deserialize;
use tower_cookies::{Cookie,  Cookies};

// struct type to represent the incoming gift, represented as JSON
#[derive(Debug, Deserialize)]
pub struct Gift {
    pub contents: String,
}

// Day 16 Task 1 wrap handler
#[debug_handler]
#[tracing::instrument(name = "Day 16, Task 1 wrap Handler")]
pub async fn day16_task1_wrap(cookies: Cookies, payload: Result<Json<Gift>, JsonRejection>) -> impl IntoResponse {   
    match payload {
        Ok(pl) => {
            cookies.add(Cookie::new("gift", pl.contents.to_string() )); 
            
            return StatusCode::OK;
        },
        Err(_) => return StatusCode::BAD_REQUEST,
    }
}

// Day 16 Task 1 unwrap handler
#[debug_handler]
#[tracing::instrument(name = "Day 16, Task 1 unwrap handler")]
pub async fn day16_task1_unwrap(cookies: Cookies) -> impl IntoResponse {
    let ck = cookies.get("Set-Cookie").unwrap();
    (StatusCode::OK, ck.to_string()).into_response()
}
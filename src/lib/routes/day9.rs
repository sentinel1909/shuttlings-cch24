// src/lib/routes/day9.rs

// dependencies
use crate::AppState;
use axum::{
    extract::{rejection::JsonRejection, Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::headers::ContentType;
use axum_extra::TypedHeader;
use axum_macros::debug_handler;
use serde::Deserialize;
use serde_json::json;

const LITERS_TO_GALLONS: f32 = 3.78541;
const LITRES_TO_PINTS: f32 = 1.759754;

// enum type to represent units
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Units {
    Liters(f32),
    Gallons(f32),
    Pints(f32),
    Litres(f32),
}

// Day 9, Task 1 handler
#[debug_handler]
#[tracing::instrument(name = "Day 9, Tasks Handler", skip(state, payload))]
pub async fn day9_tasks(
    State(state): State<AppState>,
    content_type: Option<TypedHeader<ContentType>>,
    payload: Result<Json<Units>, JsonRejection>,
) -> impl IntoResponse {
    let rate_limiter = state.rate_limiter.lock().await;
    let milk_bucket = rate_limiter.try_acquire(1);
    if !milk_bucket {
        return (StatusCode::TOO_MANY_REQUESTS, "No milk available\n").into_response();
    }
    if let Some(content_type) = content_type {
        if content_type.to_string() == "application/json" {
            match payload {
                Ok(value) => match value.0 {
                    Units::Liters(quantity) => {
                        let gallons = quantity / LITERS_TO_GALLONS;
                        return (StatusCode::OK, Json(json!({"gallons": gallons}))).into_response();
                    }
                    Units::Gallons(quantity) => {
                        let liters = quantity * LITERS_TO_GALLONS;
                        return (StatusCode::OK, Json(json!({"liters": liters}))).into_response();
                    }
                    Units::Litres(quantity) => {
                        let pints = quantity * LITRES_TO_PINTS;
                        return (StatusCode::OK, Json(json!({"pints": pints}))).into_response();
                    }
                    Units::Pints(quantity) => {
                        let litres = quantity / LITRES_TO_PINTS;
                        return (StatusCode::OK, Json(json!({"litres": litres}))).into_response();
                    }
                },
                Err(_) => return (StatusCode::BAD_REQUEST).into_response(),
            }
        }
    }

    (StatusCode::OK, "Milk withdrawn\n").into_response()
}

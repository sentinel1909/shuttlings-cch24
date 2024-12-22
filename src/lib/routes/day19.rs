// src/lib/routes/day19.rs

// dependencies
use crate::startup::AppState;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_macros::debug_handler;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use uuid::Uuid;

// struct type to represent the incoming JSON payload contained in the request body
#[derive(Clone, Debug, Deserialize)]
pub struct Payload {
    author: String,
    quote: String,
}

// struct type to represent the API endpoint response body
#[derive(Serialize)]
pub struct ResponseBody {
    id: Uuid,
    author: String,
    quote: String,
    created_at: DateTime<Utc>,
    version: i32,
}

// Day 19 Handler - Task 1, /19/draft endpoint
#[debug_handler]
#[tracing::instrument(name = "Day 19 Handler - Draft Endpoint", skip(state))]
pub async fn day19_post_draft(
    State(state): State<AppState>,
    Json(payload): Json<Payload>,
) -> impl IntoResponse {
    let id = Uuid::new_v4();
    let created_at: DateTime<Utc> = Utc::now();
    let version = 1;

    let query = sqlx::query("INSERT INTO quotes (id, author, quote, created_at, version) VALUES ($1, $2, $3, $4, $5) RETURNING *")
      .bind(id)
      .bind(payload.author)
      .bind(payload.quote)
      .bind(created_at)
      .bind(version)
      .fetch_one(&state.db)
      .await
      .unwrap();

    let id: Uuid = query.try_get("id").unwrap();
    let author: String = query.try_get("author").unwrap();
    let quote: String = query.try_get("quote").unwrap();
    let created_at: DateTime<Utc> = query.try_get("created_at").unwrap();
    let version = query.try_get("version").unwrap();

    let response_body = ResponseBody {
        id,
        author,
        quote,
        created_at,
        version,
    };

    (StatusCode::CREATED, Json(response_body))
}

// Day 19 Handler - reset endpoint
#[debug_handler]
#[tracing::instrument(name = "Day 19 Handler - Reset Endpoint", skip(state))]
pub async fn day19_get_reset(State(state): State<AppState>) -> impl IntoResponse {
    let _query = sqlx::query("DELETE FROM quotes")
        .execute(&state.db)
        .await
        .unwrap();

    StatusCode::OK
}

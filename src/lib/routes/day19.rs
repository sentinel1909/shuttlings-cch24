// src/lib/routes/day19.rs

// dependencies
use crate::startup::AppState;
use axum::{
    extract::{Json, Path, State},
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

// struct type to represent the API endpoint response when an item has been deleted
#[derive(Serialize)]
pub struct DeletedResponseBody {
    quote: String,
}

// Day 19 Handler - Task 1, /19/draft endpoint, adds an entry into the database
#[debug_handler]
#[tracing::instrument(name = "Day 19 Handler - /19/draft Endpoint", skip(state))]
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

// Day 19 Handler - reset endpoint, resets the database and removes all entries
#[debug_handler]
#[tracing::instrument(name = "Day 19 Handler - /19/reset Endpoint", skip(state))]
pub async fn day19_get_reset(State(state): State<AppState>) -> impl IntoResponse {
    let _query = sqlx::query("DELETE FROM quotes")
        .execute(&state.db)
        .await
        .unwrap();

    StatusCode::OK
}

// Day 19 Handler - cite/{id} endpoint, returns the quote with the specified id
#[debug_handler]
#[tracing::instrument(name = "Day 19 Handler - /19/cite/{id} Endpoint", skip(state))]
pub async fn day19_get_cite_by_id(
    State(state): State<AppState>,
    Path(cite_id): Path<Uuid>,
) -> impl IntoResponse {
    let id = cite_id;
    let query = sqlx::query("SELECT * FROM quotes WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .unwrap();

    match query {
        Some(query) => {
            let id = query.try_get("id").unwrap();
            let author = query.try_get("author").unwrap();
            let quote = query.try_get("quote").unwrap();
            let created_at = query.try_get("created_at").unwrap();
            let version = query.try_get("version").unwrap();

            let response_body = ResponseBody {
                id,
                author,
                quote,
                created_at,
                version,
            };

            (StatusCode::OK, Json(response_body)).into_response()
        }
        None => (StatusCode::NOT_FOUND, "".to_string()).into_response(),
    }
}

// Day 19 Handler - cite/{id} endpoint, deletes the quote with the specified id and returns the quote
#[debug_handler]
#[tracing::instrument(name = "Day 19 Handler - /19/remove/{id} Endpoint", skip(state))]
pub async fn day19_delete_by_id(
    State(state): State<AppState>,
    Path(delete_id): Path<Uuid>,
) -> impl IntoResponse {
    let id = delete_id;
    let query = sqlx::query("DELETE FROM quotes WHERE id = $1 RETURNING quote")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .unwrap();

    match query {
        Some(query) => {
            let quote = query.try_get("quote").unwrap();

            let deleted_response_body = DeletedResponseBody { quote };

            (StatusCode::OK, Json(deleted_response_body)).into_response()
        }

        None => (StatusCode::NOT_FOUND, "".to_string()).into_response(),
    }
}

// Day 19 Handler - cite/{id} endpoint, deletes the quote with the specified id and returns the quote
#[debug_handler]
#[tracing::instrument(name = "Day 19 Handler - /19/undo/{id} Endpoint", skip(state))]
pub async fn day19_update_by_id(
    State(state): State<AppState>,
    Path(undo_id): Path<Uuid>,
    Json(payload): Json<Payload>,
) -> impl IntoResponse {
    let id = undo_id;
    let revised_author = payload.author;
    let revised_quote = payload.quote;
    let query = sqlx::query("UPDATE quotes SET (author, quote, version) = ($1, $2, version+1) WHERE id = $3 RETURNING *")
        .bind(revised_author)
        .bind(revised_quote)
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .unwrap();

    match query {
        Some(query) => {
            let id = query.try_get("id").unwrap();
            let author = query.try_get("author").unwrap();
            let quote = query.try_get("quote").unwrap();
            let created_at = query.try_get("created_at").unwrap();
            let version = query.try_get("version").unwrap();

            let response_body = ResponseBody {
                id,
                author,
                quote,
                created_at,
                version,
            };

            (StatusCode::OK, Json(response_body)).into_response()
        }
        None => (StatusCode::NOT_FOUND, "".to_string()).into_response(),
    }
}

// src/lib/routes/day16.rs

// dependencies
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use axum_macros::debug_handler;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde_json::Value;
use tower_cookies::{Cookie, Cookies};

// Day 16 Task 1 - wrap endpoint
#[debug_handler]
#[tracing::instrument(name = "Day 16, Task 1 Handler - Wrap Endpoint")]
pub async fn day16_post_wrap(cookies: Cookies, Json(payload): Json<Value>) -> impl IntoResponse {
    let jwt = encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(b"the_secret_key"),
    )
    .unwrap();
    cookies.add(Cookie::new("gift", jwt));
    StatusCode::OK
}

// Day 16 Task 1 - unwrap endpoint
#[debug_handler]
#[tracing::instrument(name = "Day 16, Task 1 Handler - Unwrap Endpoint")]
pub async fn day16_get_unwrap(cookies: Cookies) -> Result<Json<Value>, StatusCode> {
    let ck = cookies.get("gift").ok_or(StatusCode::BAD_REQUEST)?;
    let jwt = ck.value();
    let mut validation = Validation::default();
    validation.required_spec_claims.remove("exp");
    let response_body = decode(
        jwt,
        &DecodingKey::from_secret(b"the_secret_key"),
        &validation,
    )
    .unwrap();
    Ok(Json(response_body.claims))
}

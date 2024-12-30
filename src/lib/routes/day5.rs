// src/lib/routes/day5.rs

// dependencies
use axum::{
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use axum_macros::debug_handler;
use cargo_manifest::Manifest;
use itertools::Itertools;
use serde::Deserialize;

// Day 5 data structure - struct type to represent Orders
#[derive(Debug, Deserialize)]
struct Orders {
    orders: Vec<toml::Value>,
}
// Day 5 data structure - struct type to represent an order
#[derive(Debug, Deserialize)]
struct Order {
    item: String,
    quantity: u32,
}

// Day 5 Tasks handler
#[debug_handler]
#[tracing::instrument(name = "Day 5 Tasks Handler", skip(body))]
pub async fn day5_tasks(headers: HeaderMap, body: String) -> impl IntoResponse {
    let Some(package) = match headers
        .get("Content-Type")
        .and_then(|header| header.to_str().ok())
    {
        Some("application/toml") => toml::from_str::<Manifest>(&body).ok(),
        Some("application/yaml") => serde_yaml::from_str::<Manifest>(&body).ok(),
        Some("application/json") => serde_json::from_str::<Manifest>(&body).ok(),
        _ => return StatusCode::UNSUPPORTED_MEDIA_TYPE.into_response(),
    }
    .and_then(|man| man.package) else {
        return (StatusCode::BAD_REQUEST, "Invalid manifest").into_response();
    };

    if !package
        .keywords
        .and_then(|keys| {
            keys.as_local()
                .map(|keys| keys.iter().any(|key| key == "Christmas 2024"))
        })
        .unwrap_or(false)
    {
        return (StatusCode::BAD_REQUEST, "Magic keyword not provided").into_response();
    }

    let orders = match package
        .metadata
        .and_then(|meta| meta.try_into::<Orders>().ok())
    {
        Some(Orders { orders }) => orders
            .into_iter()
            .filter_map(|order| order.try_into::<Order>().ok())
            .collect::<Vec<_>>(),
        _ => return StatusCode::NO_CONTENT.into_response(),
    };

    if orders.is_empty() {
        return StatusCode::NO_CONTENT.into_response();
    }

    orders
        .into_iter()
        .map(|order| format!("{}: {}", order.item, order.quantity))
        .join("\n")
        .into_response()
}

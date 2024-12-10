// src/lib/routes/day5.rs

// dependencies
use axum::{http::StatusCode, response::IntoResponse};
use axum_macros::debug_handler;
use serde::Deserialize;

// module which enables finer grained deserialization of toml values
mod serde_toml_value {
    use serde::{Deserialize, Deserializer};
    use toml::Value;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Value>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let values = Vec::<Value>::deserialize(deserializer)?;
        Ok(values)
    }
}

// struct to represent an incoming order
#[derive(Deserialize)]
struct GiftOrder {
    package: Package,
}

// struct type to represent a package
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    authors: Vec<String>,
    keywords: Vec<String>,
    metadata: Metadata,
}

// struct type to represent an order
#[derive(Debug, Deserialize)]
struct Metadata {
    #[serde(with = "serde_toml_value")]
    orders: Vec<toml::Value>, 
}

// Day 5, Task 1 handler
#[debug_handler]
#[tracing::instrument(name = "Day 5, Task 1", skip(body))]
pub async fn day5_task1(body: String) -> impl IntoResponse {
    let parsed: Result<GiftOrder, _> = toml::from_str(&body);
    if let Ok(parsed) = parsed {
        let valid_orders: Vec<String> = parsed
            .package
            .metadata
            .orders
            .into_iter()
            .filter_map(|order| match (order.get("item"), order.get("quantity")) {
                (Some(toml::Value::String(item)), Some(toml::Value::Integer(quantity))) => {
                    Some(format!("{}: {}", item, quantity))
                }
                _ => None,
            })
            .collect();

        if valid_orders.is_empty() {
            StatusCode::NO_CONTENT.into_response()
        } else {
            valid_orders.join("\n").into_response()
        }
    } else {
        StatusCode::NO_CONTENT.into_response()
    }
}

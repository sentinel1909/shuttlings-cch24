// src/lib/routes/day5.rs

// dependencies
use axum::{http::StatusCode, response::IntoResponse};
use axum_macros::debug_handler;
use cargo_manifest::Manifest;
use serde::Deserialize;
use std::str::FromStr;

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

// Day 5 data structure - struct type to represent an incoming order
#[derive(Deserialize)]
struct GiftOrder {
    package: Package,
}

// Day 5 data structure - struct type to represent a package
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    authors: Vec<String>,
    keywords: Vec<String>,
    metadata: Metadata,
}

// Day 5 data structure - struct type to represent an order metadata
#[derive(Debug, Deserialize)]
struct Metadata {
    #[serde(with = "serde_toml_value")]
    orders: Vec<toml::Value>,
}

// Day 5 Tasks handler
#[debug_handler]
#[tracing::instrument(name = "Day 5 Tasks Handler", skip(body))]
pub async fn day5_task1(body: String) -> impl IntoResponse {
    let manifest = match Manifest::from_str(&body) {
        Ok(mfst) => mfst,
        Err(_) => return Err((StatusCode::BAD_REQUEST, "Invalid manifest").into_response()),
    };

    let package = match manifest.package {
        Some(pkg) => pkg,
        None => return Err((StatusCode::BAD_REQUEST, "Invalid package format").into_response()),
    };

    let keywords = match package.keywords {
        Some(kw) => kw,
        None => return Err((StatusCode::BAD_REQUEST, "Magic keyword not provided").into_response()),
    };

    let keywords = match keywords.as_local() {
        Some(kw) => kw,
        None => return Err((StatusCode::BAD_REQUEST, "Invalid keywords format").into_response()),
    };

    if !keywords.iter().any(|keyword| keyword == "Christmas 2024") {
        return Err((StatusCode::BAD_REQUEST, "Magic keyword not provided").into_response());
    }

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
            return Err(StatusCode::NO_CONTENT.into_response());
        } else {
            Ok(valid_orders.join("\n").into_response())
        }
    } else {
        return Err(StatusCode::NO_CONTENT.into_response());
    }
}

// src/main.rs

// dependencies
use axum::{
    extract::Query,
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use serde::Deserialize;

// struct type to represent the Query parameters for Day 2, Task 1
#[derive(Deserialize)]
struct Address {
    from: String,
    key: String,
}

// Day -1, Task 1 handler
async fn dayminus1_task1() -> impl IntoResponse {
    "Hello, bird!"
}

// Day -1, Task 2 handler
async fn dayminus1_task2() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::LOCATION,
        "https://www.youtube.com/watch?v=9Gc4QTqslN4"
            .parse()
            .unwrap(),
    );
    (headers, StatusCode::FOUND)
}

// Day 2, Task 1 handler
async fn day2_task1(address: Query<Address>) -> impl IntoResponse {
    let address: Address = address.0;
    let from = address.from;
    let key = address.key;

    let from_octets: Vec<_> = from.split(".").map(|o| o.parse::<u8>().unwrap()).collect();

    let key_octets: Vec<_> = key.split(".").map(|k| k.parse::<u8>().unwrap()).collect();
    let mut dest: Vec<u8> = Vec::new();

    for (i, item) in key_octets.iter().enumerate() {
        let (sum, _) = item.overflowing_add(from_octets[i]);
        dest.push(sum);
    }

    let mut dest_str = String::new();

    for (i, item) in dest.iter().enumerate() {
        dest_str.push_str(&item.to_string());

        if i < dest.len() - 1 {
            dest_str.push('.');
        }
    }

    dest_str
}

// main function
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(dayminus1_task1))
        .route("/-1/seek", get(dayminus1_task2))
        .route("/2/dest", get(day2_task1));
    Ok(router.into())
}

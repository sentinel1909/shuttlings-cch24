// src/main.rs

// dependencies
use shuttlings_cch24::startup::build_router;

// main function
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = build_router();
    Ok(router.into())
}

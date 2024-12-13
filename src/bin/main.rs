// src/main.rs

// dependencies
use shuttlings_cch24::startup::{AppState, Application};
use shuttlings_cch24::telemetry::{get_subscriber, init_subscriber};

// main function
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    // initialize tracing
    let subscriber = get_subscriber("shuttlings-cch24".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // initialize the application state
    // application state currently holds: milk bucket quantity and refill rate for Day 9
    let app_state = AppState::new(5, 1);

    // build the application
    let app = Application::build(app_state);

    // start
    Ok(app.0.into())
}

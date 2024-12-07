// src/main.rs

// dependencies
use shuttlings_cch24::startup::Application;
use shuttlings_cch24::telemetry::{get_subscriber, init_subscriber};

// main function
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    // initialize tracing
    let subscriber = get_subscriber("shuttlings-cch24".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // build the application
    let app = Application::build();

    // start
    Ok(app.0.into())
}

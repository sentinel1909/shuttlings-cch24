// src/main.rs

// dependencies
use shuttle_shared_db::Postgres;
use shuttlings_cch24::startup::{AppState, Application};
use shuttlings_cch24::telemetry::{get_subscriber, init_subscriber};
use sqlx::PgPool;

// main function
#[shuttle_runtime::main]
async fn main(#[Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    // initialize tracing
    let subscriber = get_subscriber("shuttlings-cch24".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // database migrations
    tracing::info!("Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Unable to migrate the database.");

    // initialize the application state
    tracing::info!("Initializing the application state...");
    let milk_bucket_max_volume = 5; // max milk bucket volume, in gallons
    let milk_refill_rate = 1; // milk bucket refill rate, in gallons
    let app_state = AppState::new(milk_bucket_max_volume, milk_refill_rate, pool);

    // build the application
    tracing::info!("Building the application...");
    let app = Application::build(app_state);

    // start
    Ok(app.0.into())
}

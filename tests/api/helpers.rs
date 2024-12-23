// tests/api/helpers.rs

// dependencies
use reqwest::Client;
use shuttlings_cch24::telemetry::{get_subscriber, init_subscriber};
use shuttlings_cch24::{AppState, Application};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::env::var;
use std::io::{sink, stdout};
use std::net::TcpListener;
use std::sync::LazyLock;
use testcontainers_modules::{postgres, testcontainers::runners::AsyncRunner};

// static constant which creates one instance of tracing
static TRACING: LazyLock<()> = LazyLock::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, sink);
        init_subscriber(subscriber);
    }
});

// struct type which models a test application
#[allow(dead_code)]
pub struct TestApp {
    pub application_address: String,
    pub application_port: u16,
    pub application_client: Client,
    pub application_state: AppState,
}

pub async fn spawn_app() -> TestApp {
    // setup tracing
    LazyLock::force(&TRACING);

    // setup the database
    let _container = postgres::Postgres::default()
        .start()
        .await
        .expect("Unable to start testcontainers Postgres image.");
    let pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(
            PgConnectOptions::new()
                .host("localhost")
                .username("postgres")
                .password("postgres")
                .port(5432),
        );

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Unable to run the database migrations");

    // build the app for testing
    let milk_capacity = 5;
    let milk_refill_rate = 1;
    let app_state = AppState::new(milk_capacity, milk_refill_rate, pool);
    let application = Application::build(app_state.clone());
    let listener = TcpListener::bind("localhost:0").expect("Failed to bind port.");
    let addr = listener.local_addr().unwrap();
    let port = addr.port();

    // run the app
    tokio::spawn(application.run_until_stopped(addr));

    // configure the base, empty API client for testing
    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    TestApp {
        application_address: format!("http://localhost:{}", port),
        application_port: port,
        application_client: client,
        application_state: app_state,
    }
}

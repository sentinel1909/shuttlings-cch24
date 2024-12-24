// tests/api/helpers.rs

// dependencies
use reqwest::Client;
use shuttlings_cch24::telemetry::{get_subscriber, init_subscriber};
use shuttlings_cch24::{AppState, Application};
use sqlx::{postgres::PgConnectOptions, Connection, Executor, PgConnection, PgPool};
use std::env::var;
use std::io::{sink, stdout};
use std::net::TcpListener;
use std::sync::LazyLock;
use testcontainers_modules::{postgres, testcontainers::runners::AsyncRunner};
use uuid::Uuid;

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

// struct type to represent the test database settings
#[derive(Clone, Debug)]
struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

// methods for the DatabaseSettings type
impl DatabaseSettings {
    pub fn new() -> Self {
        DatabaseSettings {
            username: "postgres".into(),
            password: "postgres".into(),
            port: 5432,
            host: "localhost".into(),
            database_name: Uuid::new_v4().to_string(),
        }
    }

    pub fn without_db(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password)
            .port(self.port)
    }

    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db().database(&self.database_name)
    }
}

// function to configure the testing database
async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let _container = postgres::Postgres::default()
        .start()
        .await
        .expect("Unable to start testcontainers Postgres image.");

    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres.");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

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
    let db_config = DatabaseSettings::new();
    let pool = configure_database(&db_config).await;

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

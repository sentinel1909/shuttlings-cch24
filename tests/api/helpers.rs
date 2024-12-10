// tests/api/helpers.rs

// dependencies
use reqwest::Client;
use shuttlings_cch24::Application;
use std::net::TcpListener;

// struct type which models a test application
pub struct TestApp {
    pub application_address: String,
    pub application_port: u16,
    pub application_client: Client,
}

pub async fn spawn_app() -> TestApp {
    // build the app for testing
    let application = Application::build();
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
    }
}

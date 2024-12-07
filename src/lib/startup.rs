// src/lib/startup.rs

// dependencies
use crate::routes::handlers::{day2_task1, day2_task2, dayminus1_task1, dayminus1_task2};
use crate::telemetry::MakeRequestUuid;
use axum::{http::HeaderName, routing::get, Router};
use tower::ServiceBuilder;
use tower_http::{
    request_id::{PropagateRequestIdLayer, SetRequestIdLayer},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

// struct type to represent the Shuttlings CCH24 application
#[derive(Debug, Clone)]
pub struct Application(pub Router);

// methods for the Application type
impl Application {
    // function to build and return a router type, configured with all the necessary routes and handlers
    pub fn build() -> Self {
        // define the tracing layer
        let trace_layer = TraceLayer::new_for_http()
            .make_span_with(
                DefaultMakeSpan::new()
                    .include_headers(true)
                    .level(Level::INFO),
            )
            .on_response(DefaultOnResponse::new().include_headers(true));
            let x_request_id = HeaderName::from_static("x-request-id");
        let router = Router::new()
            .route("/", get(dayminus1_task1))
            .route("/-1/seek", get(dayminus1_task2))
            .route("/2/dest", get(day2_task1))
            .route("/2/key", get(day2_task2))
            .layer(
                ServiceBuilder::new()
                    .layer(SetRequestIdLayer::new(
                        x_request_id.clone(),
                        MakeRequestUuid,
                    ))
                    .layer(trace_layer)
                    .layer(PropagateRequestIdLayer::new(x_request_id)),
            );
        Self(router)
    }
}

// src/lib/startup.rs

// dependencies
use crate::routes::day12::Game;
use crate::routes::day12::{day12_get_board_state, day12_post_play_game, day12_post_reset_board};
use crate::routes::day16::{day16_post_wrap, day16_get_unwrap};
use crate::routes::day2::{day2_task1, day2_task2};
use crate::routes::day5::day5_task1;
use crate::routes::day9::day9_tasks;
use crate::routes::day_minus_one::{day_minus_one_task1, day_minus_one_task2};
use crate::telemetry::MakeRequestUuid;
use axum::{
    http::HeaderName,
    routing::{get, post},
    Router,
};
use axum_macros::FromRef;
use leaky_bucket::RateLimiter;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tokio::time::Duration;
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::{
    request_id::{PropagateRequestIdLayer, SetRequestIdLayer},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;
// struct type to represent application state
#[derive(Debug, Clone, FromRef)]
pub struct AppState {
    pub rate_limiter: Arc<RwLock<RateLimiter>>,
    pub game: Arc<RwLock<Game>>,
}

// methods for the AppState type
impl AppState {
    pub fn new(max: usize, refill: u64) -> Self {
        let rate_limiter = RateLimiter::builder()
            .initial(max)
            .max(max)
            .interval(Duration::from_secs(refill))
            .build();

        let game = Game::default();

        Self {
            rate_limiter: Arc::new(RwLock::new(rate_limiter)),
            game: Arc::new(RwLock::new(game)),
        }
    }
}

// struct type to represent the Shuttlings CCH24 application
#[derive(Debug, Clone)]
pub struct Application(pub Router);

// methods for the Application type
impl Application {
    // function to build and return a router type, configured with all the necessary routes and handlers
    pub fn build(state: AppState) -> Self {
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
            .route("/", get(day_minus_one_task1))
            .route("/-1/seek", get(day_minus_one_task2))
            .route("/2/dest", get(day2_task1))
            .route("/2/key", get(day2_task2))
            .route("/5/manifest", post(day5_task1))
            .route("/9/milk", post(day9_tasks))
            .route("/12/board", get(day12_get_board_state))
            .route("/12/reset", post(day12_post_reset_board))
            .route("/12/place/:team/:column", post(day12_post_play_game))
            .route("/16/wrap", post(day16_post_wrap))
            .route("/16/unwrap", get(day16_get_unwrap))
            .with_state(state)
            .layer(CookieManagerLayer::new())
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

    // utility function to run the application until stopped, to facilitate testing
    pub async fn run_until_stopped(self, addr: SocketAddr) {
        let listener = TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, self.0).await.unwrap();
    }
}

// src/lib/startup.rs

// dependencies
use crate::routes::day12::Game;
use crate::routes::day12::{day12_post_place_item, day12_post_reset_board, day_12_get_board_state};
use crate::routes::day16::{day16_get_unwrap, day16_post_wrap};
use crate::routes::day19::{
    day19_cite_by_id, day19_draft, day19_remove_by_id, day19_reset, day19_undo_by_id,
};
use crate::routes::day2::{day2_task1, day2_task2};
use crate::routes::day23::{day23_task2, day23_task3, day23_task4};
use crate::routes::day5::day5_task1;
use crate::routes::day9::day9_tasks;
use crate::routes::day_minus_one::{day_minus_one_task1, day_minus_one_task2};
use crate::telemetry::MakeRequestUuid;
use axum::{
    http::HeaderName,
    routing::{delete, get, post, put},
    Router,
};
use axum_macros::FromRef;
use leaky_bucket::RateLimiter;
use sqlx::PgPool;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tokio::time::Duration;
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::{
    request_id::{PropagateRequestIdLayer, SetRequestIdLayer},
    services::ServeDir,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;
// struct type to represent application state
#[derive(Debug, Clone, FromRef)]
pub struct AppState {
    pub rate_limiter: Arc<RwLock<RateLimiter>>,
    pub game: Arc<RwLock<Game>>,
    pub db: PgPool,
}

// methods for the AppState type
impl AppState {
    pub fn new(max: usize, refill: u64, pool: PgPool) -> Self {
        let rate_limiter = RateLimiter::builder()
            .initial(max)
            .max(max)
            .interval(Duration::from_secs(refill))
            .build();

        let game = Game::default();

        Self {
            rate_limiter: Arc::new(RwLock::new(rate_limiter)),
            game: Arc::new(RwLock::new(game)),
            db: pool,
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

        // serve the assets for Day 23
        let assets_service = ServiceBuilder::new()
            .layer(&trace_layer)
            .service(ServeDir::new("assets"));

        let x_request_id = HeaderName::from_static("x-request-id");

        // api routes
        let api_routes = Router::new()
            .route("/", get(day_minus_one_task1))
            .route("/-1/seek", get(day_minus_one_task2))
            .route("/2/dest", get(day2_task1))
            .route("/2/key", get(day2_task2))
            .route("/5/manifest", post(day5_task1))
            .route("/9/milk", post(day9_tasks))
            .route("/12/board", get(day_12_get_board_state))
            .route("/12/reset", post(day12_post_reset_board))
            .route("/12/place/:team/:column", post(day12_post_place_item))
            .route("/16/wrap", post(day16_post_wrap))
            .route("/16/unwrap", get(day16_get_unwrap))
            .route("/19/draft", post(day19_draft))
            .route("/19/reset", post(day19_reset))
            .route("/19/cite/:id", get(day19_cite_by_id))
            .route("/19/remove/:id", delete(day19_remove_by_id))
            .route("/19/undo/:id", put(day19_undo_by_id))
            .route("/23/star", get(day23_task2))
            .route("/23/present/:color", get(day23_task3))
            .route("/23/ornament/:state/:n", get(day23_task4))
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

        // final router, including api routes and assets service
        let router = Router::new()
            .nest_service("/", api_routes)
            .nest_service("/assets", assets_service);
        Self(router)
    }

    // utility function to run the application until stopped, to facilitate testing
    pub async fn run_until_stopped(self, addr: SocketAddr) {
        let listener = TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, self.0).await.unwrap();
    }
}

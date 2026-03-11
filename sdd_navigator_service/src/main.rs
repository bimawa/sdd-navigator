mod handlers;
mod models;
mod scanner;
mod state;

use axum::{Router, response::Html, routing::{get, post}};
use std::env;
use tower_http::cors::CorsLayer;
use tracing_subscriber::EnvFilter;

use handlers::{get_metrics, get_requirements, get_uncovered, post_scan};
use state::build_state;

static INDEX_HTML: &str = include_str!("../assets/index.html");

async fn serve_index() -> Html<&'static str> {
    Html(INDEX_HTML)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse().unwrap()))
        .init();

    let scan_root = env::var("SCAN_ROOT").unwrap_or_default();

    let shared_state = if scan_root.is_empty() {
        state::build_empty()
    } else {
        tracing::info!("Initial scan: {}", scan_root);
        build_state(&scan_root)
    };

    let app = Router::new()
        .route("/", get(serve_index))
        .route("/metrics", get(get_metrics))
        .route("/requirements", get(get_requirements))
        .route("/uncovered", get(get_uncovered))
        .route("/scan", post(post_scan))
        .layer(CorsLayer::permissive())
        .with_state(shared_state);

    let addr = "0.0.0.0:3000";
    tracing::info!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

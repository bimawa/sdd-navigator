use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::models::ScanRequest;
use crate::state::{rebuild_state, SharedState};

pub async fn get_metrics(State(state): State<SharedState>) -> impl IntoResponse {
    let s = state.read().await;
    (StatusCode::OK, Json(s.metrics.clone()))
}

pub async fn get_requirements(State(state): State<SharedState>) -> impl IntoResponse {
    let s = state.read().await;
    (StatusCode::OK, Json(s.requirements.clone()))
}

pub async fn get_uncovered(State(state): State<SharedState>) -> impl IntoResponse {
    let s = state.read().await;
    (StatusCode::OK, Json(s.uncovered.clone()))
}

pub async fn post_scan(
    State(state): State<SharedState>,
    Json(req): Json<ScanRequest>,
) -> impl IntoResponse {
    let path = req.path.trim().to_string();

    if path.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "path must not be empty" })),
        );
    }

    if !std::path::Path::new(&path).exists() {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({ "error": format!("path does not exist: {}", path) })),
        );
    }

    rebuild_state(&state, &path).await;

    let s = state.read().await;
    (StatusCode::OK, Json(json!({ "ok": true, "metrics": s.metrics })))
}

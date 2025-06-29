use axum::{Extension, response::IntoResponse};
use reqwest::StatusCode;
use std::sync::Arc;
use tracing::info;

use crate::AppState;

pub async fn ready() -> impl IntoResponse {
    (StatusCode::OK, "Ready!")
}

pub async fn health() -> impl IntoResponse {
    info!("I'm good!");
    (StatusCode::OK, "I'm good")
}

pub async fn start_game(Extension(app_state): Extension<Arc<AppState>>) -> impl IntoResponse {
    info!("Starting the game!");

    match app_state.services_ready {
        true => (StatusCode::OK, "Starting the game!"),
        false => (
            StatusCode::METHOD_NOT_ALLOWED,
            "Waiting for other services!",
        ),
    }
}

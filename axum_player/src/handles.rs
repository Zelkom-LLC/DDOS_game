use axum::response::IntoResponse;
use reqwest::StatusCode;
use tracing::info;

pub async fn ready() -> impl IntoResponse {
    (StatusCode::OK, "Ready!")
}

pub async fn health() -> impl IntoResponse {
    info!("I'm good!");
    (StatusCode::OK, "I'm good")
}

pub async fn start_game() -> impl IntoResponse {
    info!("Starting the game!");
    (StatusCode::OK, "Starting the game!")
}

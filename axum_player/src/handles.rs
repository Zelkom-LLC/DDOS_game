use axum::{Extension, response::IntoResponse};
use reqwest::StatusCode;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

use crate::{AppState, ServiceState, game::start_attack_game};

pub async fn ready() -> impl IntoResponse {
    (StatusCode::OK, "Ready!")
}

pub async fn health() -> impl IntoResponse {
    info!("I'm good!");
    (StatusCode::OK, "I'm good")
}

pub async fn start_game(
    Extension(app_state): Extension<Arc<RwLock<AppState>>>,
) -> impl IntoResponse {
    info!("Trying to start the game!");

    match app_state.read().await.service_state() {
        ServiceState::ServicesWaiting => {
            warn!("Cannot start game: services are still initializing.");
            (
                StatusCode::METHOD_NOT_ALLOWED,
                "Waiting for other services!",
            )
        }
        ServiceState::Game => {
            warn!("Cannot start game: a game is already in process.");
            (StatusCode::METHOD_NOT_ALLOWED, "Game is in process!")
        }
        ServiceState::ServicesReady => {
            info!("All services ready, starting the game...");

            let app_state_ref = app_state.clone();
            tokio::spawn(async move { start_attack_game(app_state_ref).await });
            (StatusCode::OK, "Starting the game!")
        }
    }
}

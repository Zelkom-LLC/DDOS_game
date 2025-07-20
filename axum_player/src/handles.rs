use axum::{Json, extract::Path, response::IntoResponse};
use reqwest::StatusCode;
use tracing::{error, info};

use crate::{
    GameSettings,
    game::{
        start_attack_game,
        strategy::{burn_cpu, fibonacci, fibonacci_iterative},
    },
};

pub async fn ready() -> impl IntoResponse {
    (StatusCode::OK, "Ready!")
}

pub async fn health() -> impl IntoResponse {
    info!("I'm good!");
    (StatusCode::OK, "I'm good")
}

pub async fn fib_rec(Path(attack): Path<u128>) -> impl IntoResponse {
    (StatusCode::OK, format!("Defense! - {}", fibonacci(attack)))
}

pub async fn fib_iter(Path(attack): Path<u128>) -> impl IntoResponse {
    (
        StatusCode::OK,
        format!("Defense! - {}", fibonacci_iterative(attack)),
    )
}

pub async fn burn(Path(attack): Path<usize>) -> impl IntoResponse {
    (StatusCode::OK, format!("Defense! - {}", burn_cpu(attack)))
}

pub async fn start(Json(settings): Json<GameSettings>) -> impl IntoResponse {
    info!(
        "Start the round with settings\nSettings: connections = {}, delay = {}ms, round = {}s, targets = {:?}",
        settings.connections_amount, settings.delay_ms, settings.round_sec, settings.targets
    );

    match start_attack_game(settings).await {
        Ok(_) => info!("Round was started!"),
        Err(err) => error!("{err}"),
    };

    StatusCode::OK
}

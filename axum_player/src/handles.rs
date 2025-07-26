use axum::{extract::Path, response::IntoResponse};
use reqwest::StatusCode;
use tracing::info;

use crate::game::{burn_cpu, fibonacci, fibonacci_iterative};

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

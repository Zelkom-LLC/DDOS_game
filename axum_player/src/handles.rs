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
    let res = fibonacci(attack);
    info!("fib_rec - {res}");
    (StatusCode::OK, format!("Defense! - {}", res))
}

pub async fn fib_iter(Path(attack): Path<u128>) -> impl IntoResponse {
    let res = fibonacci_iterative(attack);
    info!("fib_iter - {res}");
    (StatusCode::OK, format!("Defense! - {}", res))
}

pub async fn burn(Path(attack): Path<usize>) -> impl IntoResponse {
    let res = burn_cpu(attack);
    info!("burn - {res}");
    (StatusCode::OK, format!("Defense! - {}", res))
}

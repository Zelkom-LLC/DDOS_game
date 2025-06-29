use axum::{Router, http::StatusCode, response::IntoResponse, routing::get};
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().map_err(|e| eprintln!("{e}")).ok();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, Axum!" }))
        .route("/health", get(health));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn health() -> impl IntoResponse {
    StatusCode::OK
}

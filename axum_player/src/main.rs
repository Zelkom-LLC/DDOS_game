use axum::{Router, routing::get};
use dotenvy::dotenv;
use std::str::FromStr;
use tracing::{Level, info, warn};
use tracing_subscriber::FmtSubscriber;

use axum_player::handles::{bomb, burn, fib_iter, fib_rec, health, ready};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    dotenv().map_err(|e| warn!("{e}")).ok();

    let log_level = match std::env::var("LOG_LEVEL") {
        Ok(level) => Level::from_str(&level).unwrap_or(Level::INFO),
        Err(e) => {
            warn!("{e}");
            Level::INFO
        }
    };

    let subscriber = FmtSubscriber::builder().with_max_level(log_level).finish();

    tracing::subscriber::set_global_default(subscriber)?;

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, Axum!" }))
        .route("/health", get(health))
        .route("/ready", get(ready))
        .route("/bomb", get(bomb))
        .route("/fib_rec/{iter}", get(fib_rec))
        .route("/fib_iter/{iter}", get(fib_iter))
        .route("/burn/{iter}", get(burn));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    info!("Starting service!");
    axum::serve(listener, app).await?;

    Ok(())
}

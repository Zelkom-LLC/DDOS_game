use axum::{Extension, Router, routing::get};
use dotenvy::dotenv;
use std::{str::FromStr, sync::Arc};
use tokio::sync::RwLock;
use tracing::{Level, error, info, warn};
use tracing_subscriber::FmtSubscriber;

use axum_player::{
    AppState,
    handles::{health, ping, ready, start_game},
    poll_readiness,
};

#[tokio::main]
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

    let targets = vec![
        std::env::var("GO_ATTACKER").inspect_err(|e| error!("{}: GO_ATTACKER", e))?,
        std::env::var("PYTHON_ATTACKER").inspect_err(|e| error!("{}: PYTHON_ATTACKER", e))?,
        std::env::var("JS_ATTACKER").inspect_err(|e| error!("{}: JS_ATTACKER", e))?,
    ];

    let app_state = Arc::new(RwLock::new(AppState::default()));

    let app_state_ref = app_state.clone();

    tokio::spawn(async move { poll_readiness(&targets, app_state_ref.clone()).await });

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, Axum!" }))
        .route("/health", get(health))
        .route("/ready", get(ready))
        .route("/start_game", get(start_game))
        .route("/ping", get(ping))
        .layer(Extension(app_state));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    info!("Starting service!");
    axum::serve(listener, app).await?;

    Ok(())
}

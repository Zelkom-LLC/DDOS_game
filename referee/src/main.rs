use axum::{
    Extension, Router,
    routing::{get, post},
};
use dotenvy::dotenv;
use referee::{AppState, handlers::start, poll_readiness};
use std::{str::FromStr, sync::Arc};
use tokio::sync::RwLock;
use tracing::{Level, error, info, warn};
use tracing_subscriber::FmtSubscriber;

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

    let targets = vec![
        std::env::var("RUST_ATTACKER").inspect_err(|e| error!("{}: RUST_ATTACKER", e))?,
        std::env::var("GO_ATTACKER").inspect_err(|e| error!("{}: GO_ATTACKER", e))?,
        std::env::var("JS_ATTACKER").inspect_err(|e| error!("{}: JS_ATTACKER", e))?,
        std::env::var("PYTHON_ATTACKER").inspect_err(|e| error!("{}: PYTHON_ATTACKER", e))?,
    ];

    let app_state = Arc::new(RwLock::new(AppState::new_with_targets(targets.clone())));

    let app_state_ref = app_state.clone();

    tokio::spawn(async move { poll_readiness(&targets, app_state_ref.clone()).await });

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, Axum Referee!" }))
        .route("/start", post(start))
        .layer(Extension(app_state));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    info!("Starting service at localhost:3000!");
    axum::serve(listener, app).await?;

    Ok(())
}

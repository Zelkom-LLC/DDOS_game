use std::str::FromStr;

use axum::{Router, routing::get};
use dotenvy::dotenv;
use reqwest::Client;
use tokio::{sync::RwLock, time::Duration};
use tracing::{Level, debug, error, info, warn};
use tracing_subscriber::FmtSubscriber;

mod handles;

use handles::{health, ready, start_game};

const REQUEST_GROWING: usize = 5;
const GROWING_INTERVAL_MS: usize = 50;
const INTERVAL_BETWEEN_ATTACKS_MS: usize = 2000;

lazy_static::lazy_static! {
    static ref TARGETS_READY: RwLock<bool> = RwLock::new(false);
}

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

    tokio::spawn(async move {
        info!("Starting readiness polling of services");
        loop {
            match check_readiness(&targets).await {
                Ok(res) => {
                    if res == targets.len() {
                        let mut state = TARGETS_READY.write().await;
                        *state = true;
                        info!("Services are ready!");

                        break;
                    }
                }
                Err(e) => error!("{e}"),
            }
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    });

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, Axum!" }))
        .route("/health", get(health))
        .route("/ready", get(ready))
        .route("/start_game", get(start_game));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    info!("Starting service!");
    axum::serve(listener, app).await?;

    Ok(())
}

async fn check_readiness(targets: &Vec<String>) -> anyhow::Result<usize> {
    let client = Client::builder()
        .connect_timeout(Duration::from_secs(5))
        .build()?;
    let mut ready = 0;

    for target in targets {
        let url = format!("http://{}/ready", target);

        debug!("Request url: {url}");

        match client.get(url).send().await {
            Ok(_) => {
                info!("Target {} is ready", target);
                ready += 1;
            }
            Err(e) => error!("{e}"),
        };
    }

    Ok(ready)
}

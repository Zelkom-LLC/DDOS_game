use reqwest::Client;
use std::sync::Arc;
use tokio::{sync::RwLock, time::Duration};
use tracing::{debug, error, info, warn};

pub mod game;
pub mod handles;

const REQUEST_GROWING: usize = 5;
const GROWING_INTERVAL_MS: usize = 50;
const INTERVAL_BETWEEN_ATTACKS_MS: usize = 2000;

#[derive(Debug, Default)]
pub struct AppState {
    services_ready: bool,
    targets: Vec<String>,
}

impl AppState {
    pub fn services_ready(&self) -> bool {
        self.services_ready
    }

    pub fn targets(&self) -> &[String] {
        &self.targets
    }

    pub fn set_services_ready(&mut self, services_ready: bool) {
        self.services_ready = services_ready;
    }

    pub fn set_targets(&mut self, targets: Vec<String>) {
        self.targets = targets;
    }
}

pub async fn poll_readiness(targets: &Vec<String>, app_state: Arc<RwLock<AppState>>) {
    info!("Starting readiness polling of services");
    loop {
        match check_readiness(targets).await {
            Ok(res) if res == targets.len() => {
                let mut state = app_state.write().await;
                state.set_services_ready(true);
                info!("Services are ready!");
                break;
            }
            Ok(_) => {}
            Err(e) => error!("{e}"),
        }
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

pub async fn check_readiness(targets: &Vec<String>) -> anyhow::Result<usize> {
    let client = Client::builder()
        .connect_timeout(Duration::from_secs(5))
        .build()?;
    let mut ready = 0;

    for target in targets {
        let url = format!("http://{}/ready", target);

        debug!("Request url: {url}");

        match client.get(url).send().await {
            Ok(res) if res.status().is_success() => {
                info!("Target {} is ready", target);
                ready += 1;
            }
            Ok(res) => {
                warn!("Target is not ready. Response code is {}", res.status())
            }
            Err(e) => error!("{e}"),
        };
    }

    Ok(ready)
}

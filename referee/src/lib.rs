use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};
use strum_macros::Display;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

pub mod game;
pub mod handlers;

#[derive(Debug, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum AttackType {
    Burn(usize),
    Bomb,
    FibRec(u128),
    FibIter(u128),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameSettings {
    pub threads: usize,
    pub connections_amount: usize,
    pub round_sec: usize,
    pub attack_type: AttackType,
}

#[derive(Debug, Default)]
pub enum ServiceState {
    #[default]
    ServicesWaiting,
    ServicesReady,
    Game,
}

#[derive(Debug, Default)]
pub struct AppState {
    service_state: ServiceState,
    targets: Vec<String>,
}

impl AppState {
    pub fn new_with_targets(targets: Vec<String>) -> Self {
        Self {
            service_state: ServiceState::default(),
            targets,
        }
    }

    pub fn service_state(&self) -> &ServiceState {
        &self.service_state
    }

    pub fn targets(&self) -> &Vec<String> {
        &self.targets
    }

    pub fn set_service_state(&mut self, services_ready: ServiceState) {
        self.service_state = services_ready;
    }

    pub fn set_targets(&mut self, targets: Vec<String>) {
        self.targets = targets;
    }
}

pub async fn poll_readiness(targets: &Vec<String>, app_state: Arc<RwLock<AppState>>) {
    info!("Starting to poll readiness status of target services...");
    loop {
        match check_readiness(targets).await {
            Ok(res) if res == targets.len() => {
                let mut state = app_state.write().await;
                state.set_service_state(ServiceState::ServicesReady);
                state.set_targets(targets.clone());
                break;
            }
            Ok(ready_count) => {
                info!(
                    "Checked readiness: {}/{} services are ready. Continuing to poll...",
                    ready_count,
                    targets.len()
                );
            }
            Err(e) => error!("Failed to check readiness of services: {e}"),
        }
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
    info!("All services confirmed ready. Proceeding with next steps.");
}

pub async fn check_readiness(targets: &Vec<String>) -> anyhow::Result<usize> {
    let client = Client::builder()
        .connect_timeout(Duration::from_secs(5))
        .build()?;
    let mut ready = 0;

    for target in targets {
        let url = format!("http://{}/ready", target);

        debug!("Sending readiness request to URL: {}", url);

        match client.get(&url).send().await {
            Ok(res) if res.status().is_success() => {
                info!("Service at '{}' is ready.", target);
                ready += 1;
            }
            Ok(res) => {
                warn!(
                    "Service at '{}' responded with status code: {} (not ready yet).",
                    target,
                    res.status()
                );
            }
            Err(e) => error!("Failed to communicate with service at '{}': {}", url, e),
        };
    }

    Ok(ready)
}

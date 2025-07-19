use reqwest::Client;
use serde::Deserialize;
use std::sync::Arc;
use tokio::{sync::RwLock, time::Duration};
use tracing::{debug, error, info, warn};

pub mod game;
pub mod handles;

#[derive(Debug, Deserialize)]
pub struct GameSettings {
    pub connections_amount: usize,
    pub delay_ms: usize,
    pub round_sec: usize,
    pub difficulties: usize,
    pub targets: Vec<String>,
}

#[deprecated = "There is not anymore needed in that project"]
pub const INITIAL_INTERVAL: Duration = Duration::from_millis(1);

#[deprecated = "There is not anymore needed in that project"]
#[derive(Debug, Default)]
pub enum ServiceState {
    #[default]
    ServicesWaiting,
    ServicesReady,
    Game,
}

#[deprecated = "There is not anymore needed in that project"]
#[derive(Debug, Default)]
pub struct AppState {
    service_state: ServiceState,
    targets: Vec<String>,
}

impl AppState {
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

#[deprecated = "There is not anymore needed in that project"]
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

#[deprecated = "There is not anymore needed in that project"]
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

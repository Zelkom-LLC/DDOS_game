use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

use crate::{AppState, ServiceState};

pub async fn start_attack_game(app_state: Arc<RwLock<AppState>>) -> anyhow::Result<()> {
    let targets = app_state.read().await.targets().clone();

    info!("Targets: {:?}", targets);

    let mut app = app_state.write().await;

    app.set_service_state(ServiceState::Game);

    Ok(())
}

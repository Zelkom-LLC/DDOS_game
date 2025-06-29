use axum::Extension;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::AppState;

pub async fn start_game(
    Extension(app_state): Extension<Arc<RwLock<AppState>>>,
) -> anyhow::Result<()> {
    Ok(())
}

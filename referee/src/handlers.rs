use axum::{Extension, Json, response::IntoResponse};
use reqwest::StatusCode;
use std::sync::Arc;
use tokio::{sync::RwLock, task::JoinSet};
use tracing::{error, info};

use crate::{AppState, GameSettings, game::game};

pub async fn start(
    Extension(app_state): Extension<Arc<RwLock<AppState>>>,
    Json(settings): Json<GameSettings>,
) -> impl IntoResponse {
    let mut handles = JoinSet::new();

    let settings = Arc::new(settings);

    for target in app_state.read().await.targets().clone() {
        let settings = Arc::clone(&settings);

        handles.spawn(async move {
            info!("Start attack target on {target}");
            game(&target, &settings).await
        });
    }

    let reports = handles
        .join_all()
        .await
        .into_iter()
        .filter_map(|r| match r {
            Ok(report) => Some(report),
            Err(err) => {
                error!("{err}");
                None
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

    info!("{reports}");

    (StatusCode::OK, reports)
}

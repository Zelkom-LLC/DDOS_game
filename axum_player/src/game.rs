use futures::future::join_all;
use reqwest::Client;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info};

use crate::{AppState, GROWING_INTERVAL_MS, INITIAL_INTERVAL, MINIMAL_INTERVAL, ServiceState};

pub async fn start_attack_game(app_state: Arc<RwLock<AppState>>) -> anyhow::Result<()> {
    let targets = app_state.read().await.targets().clone();

    info!("Targets: {:?}", targets);

    let mut app = app_state.write().await;

    app.set_service_state(ServiceState::Game);

    let mut handles = Vec::new();

    let client = Client::builder()
        .build()
        .inspect_err(|e| error!("Error during building client: {e}"))?;

    for target in targets {
        let client = client.clone();
        let handle = tokio::spawn(async move {
            let target = format!("http://{target}/ping");
            attack_target(&client, &target).await;
        });

        handles.push(handle);
    }

    for handle in join_all(handles).await {
        handle.inspect_err(|e| error!("Task error: {e}")).ok();
    }

    Ok(())
}

async fn attack_target(client: &Client, target: &str) {
    let mut request_interval = INITIAL_INTERVAL;

    loop {
        match client.get(target).send().await {
            Ok(resp) => {
                info!("Запрос к {}: {}", target, resp.status());
            }
            Err(err) => {
                info!("Ошибка запроса к {}: {:?}", target, err);
            }
        }

        tokio::time::sleep(request_interval).await;

        if request_interval > MINIMAL_INTERVAL {
            request_interval -= GROWING_INTERVAL_MS;
        }

        if request_interval == MINIMAL_INTERVAL {
            request_interval = tokio::time::Duration::from_millis(1);
        }
    }
}

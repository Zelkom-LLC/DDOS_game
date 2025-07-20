use reqwest::Client;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use tokio::{task::JoinSet, time::Duration};
use tracing::{debug, error, info};

use crate::GameSettings;

pub mod strategy;

pub async fn start_attack_game(settings: GameSettings) -> anyhow::Result<()> {
    let settings = Arc::new(settings);

    let is_playing = Arc::new(AtomicBool::new(true));

    let client = {
        let client = Client::builder()
            .build()
            .inspect_err(|e| error!("Error during building client: {e}"))?;

        Arc::new(client)
    };

    let mut targets = JoinSet::new();

    for target in settings.targets.clone() {
        let is_playing = is_playing.clone();
        let client = client.clone();
        let settings = settings.clone();
        let target = Arc::new(format!("http://{target}/defence/{}", settings.difficulties));

        // Main attack task
        targets.spawn(async move {
            // Stopping algorithm
            {
                let is_playing = is_playing.clone();
                let settings = settings.clone();
                let target = target.clone();
                tokio::spawn(async move {
                    tokio::time::sleep(Duration::from_secs(settings.round_sec as u64)).await;
                    is_playing.store(false, Ordering::Relaxed);
                    info!("Time is up for target {}", target);
                });
            }

            attack_target(is_playing, settings, client, target).await;
        });
    }

    while let Some(res) = targets.join_next().await {
        if let Err(err) = res {
            error!("JoinSet error: {:?}", err);
        }
    }

    Ok(())
}

async fn attack_target(
    is_playing: Arc<AtomicBool>,
    settings: Arc<GameSettings>,
    client: Arc<Client>,
    target: Arc<String>,
) {
    let mut attackers = JoinSet::new();

    for idx in 0..settings.connections_amount {
        let is_playing = is_playing.clone();
        let client = client.clone();
        let target = target.clone();
        let settings = settings.clone();

        attackers.spawn(async move {
            debug!("Attacker with target {target} idx {idx} is playing!");

            while is_playing.load(Ordering::Relaxed) {
                match client.get(&*target).send().await {
                    Ok(resp) => {
                        if resp.status().is_client_error() {
                            info!(
                                "Request to {} succeeded with status: {}",
                                target,
                                resp.status().is_client_error()
                            );
                        }

                        if resp.status().is_server_error() {
                            info!(
                                "Request to {} succeeded with status: {}",
                                target,
                                resp.status().is_client_error()
                            );
                        }
                    }
                    Err(err) => {
                        error!("Request to {} failed with error: {:?}", target, err);
                        return;
                    }
                }

                tokio::time::sleep(tokio::time::Duration::from_millis(settings.delay_ms as u64))
                    .await;
            }
        });
    }

    while let Some(res) = attackers.join_next().await {
        if let Err(err) = res {
            error!("JoinSet error: {:?}", err);
        }
    }

    info!("Attack on target {target} is finished!");
}

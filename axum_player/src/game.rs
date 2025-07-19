use reqwest::Client;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use tracing::{debug, error, info};

use crate::GameSettings;

pub async fn start_attack_game(settings: GameSettings) -> anyhow::Result<()> {
    let is_playing = Arc::new(AtomicBool::new(true));

    let is_playing_ref = is_playing.clone();
    tokio::task::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(settings.round_sec as u64)).await;
        is_playing_ref.store(false, Ordering::Relaxed);
    });

    let settings = Arc::new(settings);

    for target in settings.targets.clone() {
        let client = {
            let client = Client::builder()
                .build()
                .inspect_err(|e| error!("Error during building client: {e}"))?;

            Arc::new(client)
        };
        let is_playing = is_playing.clone();
        let settings = settings.clone();
        tokio::spawn(async move {
            attack_target_new(
                is_playing,
                settings,
                client,
                format!("http://{target}/defence"),
            )
            .await
        });
    }

    Ok(())
}

async fn attack_target_new(
    is_playing: Arc<AtomicBool>,
    settings: Arc<GameSettings>,
    client: Arc<Client>,
    target: String,
) -> anyhow::Result<()> {
    let target = Arc::new(target);

    for idx in 0..settings.connections_amount {
        let client = client.clone();
        let target = target.clone();
        let is_playing = is_playing.clone();
        let settings = settings.clone();

        debug!("Attacker with target {target} idx {idx}");

        tokio::spawn(async move {
            while is_playing.load(Ordering::Relaxed) {
                debug!("Attacker with target {target} idx {idx} is playing!");

                match client
                    .get(&*target)
                    .body(settings.difficulties.to_string())
                    .send()
                    .await
                {
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

    Ok(())
}

pub fn fibonacci_iterative(n: usize) -> usize {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut prev = 0;
    let mut curr = 1;

    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    curr
}

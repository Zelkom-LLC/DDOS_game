use tokio::process::Command;

use crate::GameSettings;

pub async fn game(target: &str, game_settings: &GameSettings) -> anyhow::Result<String> {
    let threads = format!("-t{}", game_settings.threads);
    let connections = format!("-c{}", game_settings.connections_amount);
    let durations = format!("-d{}s", game_settings.round_sec);
    let target = format!("http://{}", target);

    let output = Command::new("wrk")
        .args(&[threads, connections, durations, target])
        .stdout(std::process::Stdio::piped())
        .spawn()?
        .wait_with_output()
        .await?;

    match output.status.success() {
        true => Ok(String::from_utf8(output.stdout)?),
        false => Ok(String::from_utf8(output.stderr)?),
    }
}

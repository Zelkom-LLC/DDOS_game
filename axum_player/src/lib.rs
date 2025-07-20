use serde::Deserialize;

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

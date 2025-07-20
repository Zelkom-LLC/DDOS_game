use serde::Deserialize;
use strum_macros::Display;

pub mod game;
pub mod handles;

#[derive(Debug, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum AttackType {
    Burn,
    Bomb,
    FibRec(u128),
    FibIter(u128),
}

#[derive(Debug, Deserialize)]
pub struct GameSettings {
    pub connections_amount: usize,
    pub delay_ms: usize,
    pub round_sec: usize,
    pub difficulties: usize,
    pub targets: Vec<String>,
}

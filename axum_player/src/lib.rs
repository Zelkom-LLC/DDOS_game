use serde::Deserialize;
use strum_macros::Display;

pub mod game;
pub mod handles;

#[derive(Debug, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum AttackType {
    Burn(usize),
    Bomb,
    FibRec(u128),
    FibIter(u128),
}

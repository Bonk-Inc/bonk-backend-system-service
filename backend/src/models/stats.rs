use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct GlobalStats {
    pub games: i64,
    pub scores: i64
}

#[derive(Serialize, Deserialize, Default)]
pub struct GameStats {
    pub scores: i64
}
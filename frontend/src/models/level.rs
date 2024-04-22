use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct LevelDTO {
    pub name: String,
    pub game_id: String,
}
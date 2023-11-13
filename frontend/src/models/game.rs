use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GameDTO {
    pub name: String
}
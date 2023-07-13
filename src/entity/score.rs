use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, types::Uuid};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Score {
    pub id: Uuid,
    pub username: String,
    pub score: i32,
    pub is_hidden: bool,
    pub game_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Game<'a> {
    pub id: Uuid,
    pub name: &'a str,
    pub created_at: NaiveDateTime
}
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::game::Game;

pub struct Score<'a> {
    pub id: Uuid,
    pub username: &'a str,
    pub game: Game<'a>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
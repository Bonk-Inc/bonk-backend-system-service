use chrono::NaiveDateTime;
#[cfg(feature = "backend")]
use diesel::{Selectable, Queryable};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg_attr(feature = "backend", derive(Queryable))]
#[derive(Serialize, Deserialize)]
pub struct Game {
    pub id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[cfg_attr(feature = "backend", derive(Queryable, Selectable))]
#[cfg_attr(feature = "backend", diesel(table_name = crate::schema::level))]
#[derive(Serialize, Clone, Deserialize, Default)]
pub struct Level {
    pub id: Uuid,
    pub name: String,
    pub game_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[cfg_attr(feature = "backend", derive(Queryable, Selectable))]
#[cfg_attr(feature = "backend", diesel(table_name = crate::schema::score))]
#[derive(Serialize, Clone, Deserialize, Default)]
pub struct Score {
    pub id: Uuid,
    pub username: String,
    #[serde(rename = "score")]
    pub highscore: i32,
    pub is_hidden: bool,
    pub level_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>
}

#[derive(Serialize, Deserialize, Default)]
pub struct GlobalStats {
    pub games: i64,
    pub scores: i64
}

#[derive(Serialize, Deserialize, Default)]
pub struct GameStats {
    pub scores: i64
}
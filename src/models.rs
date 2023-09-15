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
#[cfg_attr(feature = "backend", diesel(table_name = crate::schema::score))]
#[derive(Serialize, Deserialize)]
pub struct Score {
    pub id: Uuid,
    pub username: String,
    #[serde(rename = "score")]
    pub highscore: i32,
    pub is_hidden: bool,
    pub game_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>
}

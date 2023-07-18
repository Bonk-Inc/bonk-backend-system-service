use chrono::NaiveDateTime;
use diesel::{Queryable, Selectable, Identifiable, Associations};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Identifiable, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::game)]
pub struct Game {
    pub id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>
}

#[derive(Debug, Clone, Selectable, Identifiable, Queryable, Associations, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::score)]
#[diesel(belongs_to(Game))]
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

#[derive(Debug, Identifiable, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::user)]
#[diesel(primary_key(user_id))]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub created_at: NaiveDateTime
}
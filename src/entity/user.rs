use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct User<'a> {
    pub id: Uuid,
    pub username: &'a str,
    #[serde(skip)]
    pub password: &'a str,
    pub email: &'a str,
    pub created_at: NaiveDateTime,
}

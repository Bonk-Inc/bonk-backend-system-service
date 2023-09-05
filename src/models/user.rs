use chrono::NaiveDateTime;
use diesel::{Identifiable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::user)]
#[diesel(primary_key(user_id))]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub email: String,
    pub created_at: NaiveDateTime
}
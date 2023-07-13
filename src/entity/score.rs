use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Score<'a> {
    pub id: Uuid,
    pub username: &'a str,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
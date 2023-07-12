use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct Game<'a> {
    pub id: Uuid,
    pub name: &'a str,
    pub created_at: DateTime<Utc>
}
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct User<'a> {
    pub id: Uuid,
    pub username: &'a str,
    pub password: &'a str,
    pub email: &'a str,
    pub created_at: DateTime<Utc>
}

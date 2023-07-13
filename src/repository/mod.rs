use async_trait::async_trait;
use sqlx::{Pool, Postgres, Error};

#[async_trait]
pub trait Repository<'a> {
    type Output;

    fn new(conn: &'a Pool<Postgres>) -> Self;
    async fn find_all(self) -> Result<Vec<Self::Output>, Error>;
    async fn find(self, id: String) -> Result<Self::Output, Error>;
}
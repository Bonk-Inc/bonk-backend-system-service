use std::error::Error;

use diesel::{PgConnection, r2d2::{PooledConnection, ConnectionManager}};
use uuid::Uuid;

pub mod score_repository;

type DbError = Box<dyn Error + Send + Sync>;
type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub trait Repository<'a> {
    type Output;

    fn new(conn: DbConnection) -> Self;
    fn find_all(self) -> Result<Vec<Self::Output>, DbError>;
    fn find(self, item_id: Uuid) -> Result<Option<Self::Output>, DbError>;
    fn drop(self, item_id: Uuid) -> Result<bool, DbError>;
}
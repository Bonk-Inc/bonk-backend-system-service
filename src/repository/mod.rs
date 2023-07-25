use std::error::Error;

use actix_web::web::Json;
use diesel::{PgConnection, r2d2::{PooledConnection, ConnectionManager}};
use uuid::Uuid;

pub mod game_repository;
pub mod score_repository;

type DbError = Box<dyn Error + Send + Sync>;
type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub trait Repository {
    type Output;
    type Input;

    fn new() -> Self;
    fn find_all(self, conn: &mut DbConnection) -> Result<Vec<Self::Output>, DbError>;
    fn find(self, conn: &mut DbConnection, item_id: Uuid) -> Result<Option<Self::Output>, DbError>;
    fn drop(self, conn: &mut DbConnection, item_id: Uuid) -> Result<bool, DbError>;
    fn store(self, conn: &mut DbConnection, data: Json<Self::Input>) -> Result<Self::Output, DbError>;
    fn update(self, conn: &mut DbConnection, item_id: Uuid, data: Json<Self::Input>) -> Result<Option<Self::Output>, DbError>;
}
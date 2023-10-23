use diesel::prelude::*;

use crate::config::db::Connection;

pub mod game;
pub mod score;

pub trait Model<T, I, D> {
    fn find_all(conn: &mut Connection) -> QueryResult<Vec<T>>;
    fn find_by_id(model_id: I, conn: &mut Connection) -> QueryResult<T>;
    fn insert(data: D, conn: &mut Connection) -> QueryResult<T>;
    fn update(model_id: I, data: D, conn: &mut Connection) -> QueryResult<T>;
    fn delete(model_id: I, conn: &mut Connection) -> QueryResult<usize>;
}
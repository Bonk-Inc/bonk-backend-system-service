use diesel::prelude::*;

use crate::config::db::Connection;

pub mod game;
pub mod score;

pub trait FindAll<T> {
    fn find_all(conn: &mut Connection) -> QueryResult<Vec<T>>;
}

pub trait FindById<T, I> {
    fn find_by_id(model_id: I, conn: &mut Connection) -> QueryResult<T>;
}

pub trait Insert<D, T> {
    fn insert(data: D, conn: &mut Connection) -> QueryResult<T>;
}

pub trait Update<D, I, T> {
    fn update(model_id: I, data: D, conn: &mut Connection) -> QueryResult<T>;
}

pub trait Delete<I> {
    fn delete(model_id: I, conn: &mut Connection) -> QueryResult<usize>;
}
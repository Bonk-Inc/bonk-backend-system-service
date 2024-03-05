use babs::{models::Level, schema::level::dsl::*};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Model;

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = babs::schema::level)]
pub struct LevelDTO {
    pub name: String,
    pub game_id: Uuid,
}

impl Model<Level, Uuid, LevelDTO> for Level {
    fn find_all(conn: &mut crate::config::db::Connection) -> QueryResult<Vec<Level>> {
        level.load::<Level>(conn)
    }

    fn find_by_id(level_id: Uuid, conn: &mut crate::config::db::Connection) -> QueryResult<Level> {
        level.find(level_id).get_result::<Level>(conn)
    }

    fn insert(data: LevelDTO, conn: &mut crate::config::db::Connection) -> QueryResult<Level> {
        diesel::insert_into(level)
            .values(&data)
            .get_result::<Level>(conn)
    }

    fn update(
        level_id: Uuid,
        data: LevelDTO,
        conn: &mut crate::config::db::Connection,
    ) -> QueryResult<Level> {
        diesel::update(level)
            .filter(id.eq(level_id))
            .set(data)
            .get_result::<Level>(conn)
    }

    fn delete(level_id: Uuid, conn: &mut crate::config::db::Connection) -> QueryResult<usize> {
        diesel::delete(level).filter(id.eq(level_id)).execute(conn)
    }
}

use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    config::db::Connection,
    models::Model,
    schema::{level, level::dsl::*},
};

#[derive(Serialize, Clone, Deserialize, Default, Queryable, Selectable, ToSchema)]
#[diesel(table_name = level)]
#[diesel(belongs_to(Game))]
pub struct Level {
    pub id: Uuid,
    pub name: String,
    pub game_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = level)]
pub struct LevelDTO {
    pub name: String,
    pub game_id: Uuid,
}

impl Model<Level, Uuid, LevelDTO> for Level {
    fn find_all(conn: &mut Connection) -> QueryResult<Vec<Level>> {
        level.load::<Level>(conn)
    }

    fn find_by_id(level_id: Uuid, conn: &mut Connection) -> QueryResult<Level> {
        level.find(level_id).get_result::<Level>(conn)
    }

    fn insert(data: LevelDTO, conn: &mut Connection) -> QueryResult<Level> {
        diesel::insert_into(level)
            .values(&data)
            .get_result::<Level>(conn)
    }

    fn update(level_id: Uuid, data: LevelDTO, conn: &mut Connection) -> QueryResult<Level> {
        diesel::update(level)
            .filter(id.eq(level_id))
            .set(data)
            .get_result::<Level>(conn)
    }

    fn delete(level_id: Uuid, conn: &mut Connection) -> QueryResult<usize> {
        diesel::delete(level).filter(id.eq(level_id)).execute(conn)
    }
}

pub fn find_by_game(game: Uuid, conn: &mut Connection) -> QueryResult<Vec<Level>> {
    level
        .filter(game_id.eq(game))
        .select(Level::as_select())
        .load(conn)
}

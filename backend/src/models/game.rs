use chrono::NaiveDateTime;
use diesel::{dsl::count_star, prelude::*, AsChangeset, Insertable, QueryDsl};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    config::db::Connection,
    schema::game::{self, dsl::*},
};

#[derive(Queryable, Serialize, Identifiable, Deserialize, Default, ToSchema, Clone)]
#[diesel(table_name = game)]
pub struct Game {
    pub id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = game)]
pub struct GameDTO {
    pub name: String,
}

impl Game {
    pub fn find_all(conn: &mut Connection) -> QueryResult<Vec<Game>> {
        game.load::<Game>(conn)
    }

    pub fn find_by_id(game_id: Uuid, conn: &mut Connection) -> QueryResult<Game> {
        game.find(game_id).get_result::<Game>(conn)
    }

    pub fn insert(data: GameDTO, conn: &mut Connection) -> QueryResult<Game> {
        diesel::insert_into(game)
            .values(&data)
            .get_result::<Game>(conn)
    }

    pub fn update(model_id: Uuid, data: GameDTO, conn: &mut Connection) -> QueryResult<Game> {
        diesel::update(game)
            .filter(id.eq(model_id))
            .set(data)
            .get_result::<Game>(conn)
    }

    pub fn delete(model_id: Uuid, conn: &mut Connection) -> QueryResult<usize> {
        diesel::delete(game).filter(id.eq(model_id)).execute(conn)
    }

    pub fn count_games(conn: &mut Connection) -> QueryResult<i64> {
        game.select(count_star()).first(conn)
    }
}

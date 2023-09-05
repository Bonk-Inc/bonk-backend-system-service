use chrono::NaiveDateTime;
use diesel::{prelude::*, AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    config::db::Connection, 
    schema::game::dsl::*
};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Game {
    pub id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::game)]
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

    pub fn insert(new_game: GameDTO, conn: &mut Connection) -> QueryResult<Game> {
        diesel::insert_into(game)
            .values(&new_game)
            .get_result::<Game>(conn)
    }

    pub fn update(
        game_id: Uuid,
        updated_game: GameDTO,
        conn: &mut Connection,
    ) -> QueryResult<Game> {
        diesel::update(game)
            .filter(id.eq(game_id))
            .set(updated_game)
            .get_result::<Game>(conn)
    }

    pub fn delete(game_id: Uuid, conn: &mut Connection) -> QueryResult<usize> {
        diesel::delete(game).filter(id.eq(game_id)).execute(conn)
    }
}

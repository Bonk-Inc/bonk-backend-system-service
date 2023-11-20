pub use babs::{
    models::Game,
    schema::game::dsl::*
};
use diesel::{prelude::*, AsChangeset, Insertable, QueryDsl, dsl::count_star};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    config::db::Connection,
    models::Model,
};

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = babs::schema::game)]
pub struct GameDTO {
    pub name: String,
}

impl Model<Game, Uuid, GameDTO> for Game {
    fn find_all(conn: &mut Connection) -> QueryResult<Vec<Game>> {
        game.load::<Game>(conn)
    }

    fn find_by_id(game_id: Uuid, conn: &mut Connection) -> QueryResult<Game> {
        game.find(game_id).get_result::<Game>(conn)
    }

    fn insert(data: GameDTO, conn: &mut Connection) -> QueryResult<Game> {
        diesel::insert_into(game)
            .values(&data)
            .get_result::<Game>(conn)
    }

    fn update(model_id: Uuid, data: GameDTO, conn: &mut Connection) -> QueryResult<Game> {
        diesel::update(game)
            .filter(id.eq(model_id))
            .set(data)
            .get_result::<Game>(conn)
    }

    fn delete(model_id: Uuid, conn: &mut Connection) -> QueryResult<usize> {
        diesel::delete(game).filter(id.eq(model_id)).execute(conn)
    }
}

pub fn count_games(conn: &mut Connection) -> QueryResult<i64> {   
    game.select(count_star())
        .first(conn)
}
pub use babs::{
    models::Game,
    schema::game::dsl::*
};
use diesel::{prelude::*, AsChangeset, Insertable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    config::db::Connection,
    models::{Delete, FindAll, FindById, Insert, Update},
};

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = babs::schema::game)]
pub struct GameDTO {
    pub name: String,
}

impl FindAll<Game> for Game {
    fn find_all(conn: &mut Connection) -> QueryResult<Vec<Game>> {
        game.load::<Game>(conn)
    }
}

impl FindById<Game, Uuid> for Game {
    fn find_by_id(game_id: Uuid, conn: &mut Connection) -> QueryResult<Game> {
        game.find(game_id).get_result::<Game>(conn)
    }
}

impl Insert<GameDTO, Game> for Game {
    fn insert(data: GameDTO, conn: &mut Connection) -> QueryResult<Game> {
        diesel::insert_into(game)
            .values(&data)
            .get_result::<Game>(conn)
    }
}

impl Update<GameDTO, Uuid, Game> for Game {
    fn update(model_id: Uuid, data: GameDTO, conn: &mut Connection) -> QueryResult<Game> {
        diesel::update(game)
            .filter(id.eq(model_id))
            .set(data)
            .get_result::<Game>(conn)
    }
}

impl Delete<Uuid> for Game {
    fn delete(model_id: Uuid, conn: &mut Connection) -> QueryResult<usize> {
        diesel::delete(game).filter(id.eq(model_id)).execute(conn)
    }
}

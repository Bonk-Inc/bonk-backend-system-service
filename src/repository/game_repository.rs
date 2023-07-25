use actix_web::web::Json;
use diesel::{QueryDsl, SelectableHelper, prelude::*};

use crate::{models::Game, controller::api::game::GameForm};

use super::{Repository, DbConnection, DbError};

pub struct GameRepository;

impl GameRepository {
    
    pub fn find_by_name(&self, conn: &mut DbConnection, game_name: &String) -> Result<Option<Game>, DbError> {
        use crate::schema::game::dsl::*;

        let result = game
            .filter(name.eq(game_name))
            .first::<Game>(conn)
            .optional()?;

        Ok(result)
    }

}

impl Repository for GameRepository {
    type Output = Game;
    type Input = GameForm;

    fn new() -> Self {
       GameRepository
    }

    fn find_all(self, conn: &mut DbConnection) -> Result<Vec<Self::Output>, DbError> {
        use crate::schema::game::dsl::*;

        let result = game
            .select(Game::as_select())
            .load(conn)?;
        
        Ok(result)
    }

    fn find(self, conn: &mut DbConnection, item_id: uuid::Uuid) -> Result<Option<Self::Output>, DbError> {
        use crate::schema::game::dsl::*;

        let result = game
            .filter(id.eq(item_id))
            .first::<Game>(conn)
            .optional()?;

        Ok(result)
    }

    fn drop(self, conn: &mut DbConnection, item_id: uuid::Uuid) -> Result<bool, DbError> {
        use crate::schema::game::dsl::*;

        let result = diesel::delete(game)
        .filter(id.eq(item_id))
        .get_results::<Game>(conn)?;

        Ok(result.len() > 0)
    }

    fn store(self, conn: &mut DbConnection, data: Json<Self::Input>) -> Result<Self::Output, DbError> {
        use crate::schema::game::dsl::*;

        let result = diesel::insert_into(game)
            .values(data.0)
            .get_result::<Game>(conn)?;

        Ok(result)
    }

    fn update(self, conn: &mut DbConnection, item_id: uuid::Uuid, data: Json<Self::Input>) -> Result<Option<Self::Output>, DbError> {
        use crate::schema::game::dsl::*;

        let result = diesel::update(game)
            .filter(id.eq(item_id))
            .set(data.0)
            .get_result::<Game>(conn)
            .optional()?;

        Ok(result)
    }
}
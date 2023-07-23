use diesel::{QueryDsl, SelectableHelper, prelude::*};

use crate::models::Game;

use super::{Repository, DbConnection, DbError};

pub struct GameRepository;

impl Repository for GameRepository {
    type Output = Game;

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
}
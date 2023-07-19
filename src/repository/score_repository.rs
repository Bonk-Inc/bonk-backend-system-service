use diesel::{QueryDsl, ExpressionMethods, SelectableHelper, prelude::*};
use uuid::Uuid;

use crate::models::Score;

use super::{Repository, DbError, DbConnection};

pub struct ScoreRepository {
    pub conn: DbConnection
}

impl Repository<'_> for ScoreRepository {
    type Output = Score;
    
    fn new(conn: DbConnection) -> Self {
        ScoreRepository { conn }
    }

    fn find_all(mut self) -> Result<Vec<Self::Output>, DbError> {
        use crate::schema::score::dsl::*;

        let result = score
            .select(Score::as_select())
            .load(&mut self.conn)?;
        
        Ok(result)
    }

    fn find(mut self, item_id: Uuid) -> Result<Option<Self::Output>, DbError> {
        use crate::schema::score::dsl::*;

        let result = score
            .filter(id.eq(item_id))
            .first::<Score>(&mut self.conn)
            .optional()?;

        Ok(result)
    }

    fn drop(mut self, item_id: Uuid) -> Result<bool, DbError> {
        use crate::schema::score::dsl::*;

        let result = diesel::delete(score)
            .filter(id.eq(item_id))
            .get_results::<Score>(&mut self.conn)?;

        Ok(result.len() > 0)
    }
}
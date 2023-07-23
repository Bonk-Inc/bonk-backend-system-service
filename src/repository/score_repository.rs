use diesel::{QueryDsl, ExpressionMethods, SelectableHelper, prelude::*};
use uuid::Uuid;

use crate::models::Score;

use super::{Repository, DbError, DbConnection};

pub struct ScoreRepository;

impl Repository<'_> for ScoreRepository {
    type Output = Score;
    
    fn new() -> Self {
        ScoreRepository
    }

    fn find_all(self, conn: &mut DbConnection) -> Result<Vec<Self::Output>, DbError> {
        use crate::schema::score::dsl::*;

        let result = score
            .select(Score::as_select())
            .load(conn)?;
        
        Ok(result)
    }

    fn find(self, conn: &mut DbConnection, item_id: Uuid) -> Result<Option<Self::Output>, DbError> {
        use crate::schema::score::dsl::*;

        let result = score
            .filter(id.eq(item_id))
            .first::<Score>(conn)
            .optional()?;

        Ok(result)
    }

    fn drop(self, conn: &mut DbConnection, item_id: Uuid) -> Result<bool, DbError> {
        use crate::schema::score::dsl::*;

        let result = diesel::delete(score)
            .filter(id.eq(item_id))
            .get_results::<Score>(conn)?;

        Ok(result.len() > 0)
    }
}
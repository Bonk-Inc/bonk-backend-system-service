use actix_web::web::Json;
use diesel::{prelude::*, ExpressionMethods, QueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::{controller::api::score::ScoreForm, models::Score};

use super::{DbConnection, DbError, Repository};

pub struct ScoreRepository;

impl Repository for ScoreRepository {
    type Output = Score;
    type Input = ScoreForm;

    fn new() -> Self {
        ScoreRepository
    }

    fn find_all(self, conn: &mut DbConnection) -> Result<Vec<Self::Output>, DbError> {
        use crate::schema::score::dsl::*;

        let result = score.select(Score::as_select()).load(conn)?;

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

    fn store(self, conn: &mut DbConnection, data: Json<Self::Input>) -> Result<Self::Output, DbError> {
        use crate::schema::score::dsl::*;

        let result = diesel::insert_into(score)
            .values(data.0)
            .get_result::<Score>(conn)?;

        Ok(result)
    }

    fn update(self, conn: &mut DbConnection, item_id: Uuid, data: Json<Self::Input>) -> Result<Option<Self::Output>, DbError> {
        use crate::schema::score::dsl::*;

        let result = diesel::update(score)
            .filter(id.eq(item_id))
            .set(data.0)
            .get_result::<Score>(conn)
            .optional()?;

        Ok(result)
    }
}

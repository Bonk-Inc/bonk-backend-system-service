pub use babs::{
    models::Score,
    schema::{score, score::dsl::*}
};
use diesel::{prelude::*, AsChangeset, Insertable, dsl::count_star};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    config::db::Connection,
    models::Model,
};

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = babs::schema::score)]
pub struct ScoreDTO {
    pub username: String,
    #[serde(rename = "score")]
    pub highscore: i32,
    pub is_hidden: bool,
    pub game_id: Uuid
}

impl Model<Score, Uuid, ScoreDTO> for Score {
    fn find_all(conn: &mut Connection) -> QueryResult<Vec<Score>> {
        score.load::<Score>(conn)
    }

    fn find_by_id(score_id: Uuid, conn: &mut Connection) -> QueryResult<Score> {
        score.find(score_id).get_result::<Score>(conn)
    }

    fn insert(new_score: ScoreDTO, conn: &mut Connection) -> QueryResult<Score> {
        diesel::insert_into(score)
            .values(&new_score)
            .get_result::<Score>(conn)
    }

    fn update(score_id: Uuid, updated_score: ScoreDTO, conn: &mut Connection) -> QueryResult<Score> {
        diesel::update(score)
            .filter(id.eq(score_id))
            .set(updated_score)
            .get_result::<Score>(conn)
    }

    fn delete_many(score_ids: Vec<Uuid>, conn: &mut Connection) -> QueryResult<usize> {
        diesel::delete(score)
            .filter(id.eq_any(score_ids))
            .execute(conn)
    }
}

pub fn find_by_game(game: Uuid, include_hidden: bool, conn: &mut Connection) -> QueryResult<Vec<Score>> {
    let mut query = score::table.into_boxed(); 
    query = query.filter(game_id.eq(game));

    if !include_hidden {
        query = query.filter(is_hidden.eq(false));
    }
        
    query.select(Score::as_select())
        .load(conn)
}

pub fn count_score(game_uuid: Option<Uuid>, conn: &mut Connection) -> QueryResult<i64> {
    let mut query = score::table.into_boxed();
    
    if let Some(value) = game_uuid {
        query = query.filter(game_id.eq(value));
    }

    query.select(count_star())
        .first(conn)
}
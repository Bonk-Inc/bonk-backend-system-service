pub use babs::{
    models::Score,
    schema::{score, score::dsl::*}
};
use diesel::{prelude::*, AsChangeset, Insertable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    config::db::Connection,
    models::{Delete, FindAll, FindById, Insert, Update},
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

impl FindAll<Score> for Score {
    fn find_all(conn: &mut Connection) -> QueryResult<Vec<Score>> {
        score.load::<Score>(conn)
    }
}

impl FindById<Score, Uuid> for Score {
    fn find_by_id(score_id: Uuid, conn: &mut Connection) -> QueryResult<Score> {
        score.find(score_id).get_result::<Score>(conn)
    }
}

impl Insert<ScoreDTO, Score> for Score {
    fn insert(new_score: ScoreDTO, conn: &mut Connection) -> QueryResult<Score> {
        diesel::insert_into(score)
            .values(&new_score)
            .get_result::<Score>(conn)
    }   
}

impl Update<ScoreDTO, Uuid, Score> for Score {
    fn update(score_id: Uuid, updated_score: ScoreDTO, conn: &mut Connection) -> QueryResult<Score> {
        diesel::update(score)
            .filter(id.eq(score_id))
            .set(updated_score)
            .get_result::<Score>(conn)
    }   
}

impl Delete<Uuid> for Score {
    fn delete(score_id: Uuid, conn: &mut Connection) -> QueryResult<usize> {
        diesel::delete(score)
            .filter(id.eq(score_id))
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
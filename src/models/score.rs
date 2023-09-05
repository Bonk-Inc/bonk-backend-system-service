use chrono::NaiveDateTime;
use diesel::{prelude::*, Queryable, AsChangeset, Insertable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    config::db::Connection,
    schema::score::dsl::*
};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::score)]
pub struct Score {
    pub id: Uuid,
    pub username: String,
    #[serde(rename = "score")]
    pub highscore: i32,
    pub is_hidden: bool,
    pub game_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::score)]
pub struct ScoreDTO {
    pub username: String,
    pub highscore: i32,
    pub is_hidden: bool,
    pub game_id: Uuid
}

impl Score {
    pub fn find_all(conn: &mut Connection) -> QueryResult<Vec<Score>> {
        score.load::<Score>(conn)
    }

    pub fn find_by_id(score_id: Uuid, conn: &mut Connection) -> QueryResult<Score> {
        score.find(score_id).get_result::<Score>(conn)
    }

    pub fn find_by_game(game: Uuid, include_hidden: bool, conn: &mut Connection) -> QueryResult<Vec<Score>> {
        score.filter(game_id.eq(game))
            .filter(is_hidden.eq(include_hidden))
            .select(Score::as_select())
            .load(conn)
    }

    pub fn insert(new_score: ScoreDTO, conn: &mut Connection) -> QueryResult<Score> {
        diesel::insert_into(score)
            .values(&new_score)
            .get_result::<Score>(conn)
    }

    pub fn update(score_id: Uuid, updated_score: ScoreDTO, conn: &mut Connection) -> QueryResult<Score> {
        diesel::update(score)
            .filter(id.eq(score_id))
            .set(updated_score)
            .get_result::<Score>(conn)
    }

    pub fn delete(score_id: Uuid, conn: &mut Connection) -> QueryResult<usize> {
        diesel::delete(score)
            .filter(id.eq(score_id))
            .execute(conn)
    }
}
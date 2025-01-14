use chrono::NaiveDateTime;
use diesel::{dsl::count_star, prelude::*, result::Error, AsChangeset, Insertable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    config::db::Connection,
    models::{game::Game, level::Level, user::User},
    schema::score::{self, dsl::*},
};

#[derive(Associations, Identifiable, Queryable, Selectable)]
#[diesel(table_name = score)]
#[diesel(belongs_to(Level))]
#[diesel(belongs_to(User))]
pub struct Score {
    pub id: Uuid,
    pub username: Option<String>,
    pub highscore: i32,
    pub is_hidden: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub level_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
}

#[derive(Insertable, AsChangeset, Deserialize, ToSchema)]
#[diesel(table_name = score)]
pub struct ScoreForm {
    pub username: Option<String>,
    #[serde(rename = "score")]
    pub highscore: i32,
    pub is_hidden: bool,
    pub level_id: Uuid,
    pub user_id: Option<Uuid>,
}

#[derive(Serialize, ToSchema)]
pub struct ScoreDto {
    pub id: Uuid,
    pub score: i32,
    pub is_hidden: bool,
    pub username: Option<String>,
    pub level: Option<Level>,
    pub user: Option<User>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl ScoreDto {
    fn new(score_value: Score, conn: &mut PgConnection) -> Self {
        ScoreDto {
            id: score_value.id,
            score: score_value.highscore,
            is_hidden: score_value.is_hidden,
            username: score_value.username,
            user: score_value
                .user_id
                .map_or_else(|| None, |i: Uuid| Some(User::find_by_id(i, conn).unwrap())),
            level: score_value
                .level_id
                .map_or_else(|| None, |i| Some(Level::find_by_id(i, conn).unwrap())),
            created_at: score_value.created_at,
            updated_at: score_value.updated_at,
        }
    }
}

impl Score {
    /// Fetchets all the scores in the database by looking up all the levels related to the given game. And using 
    /// the levels to fetch all the scores.
    pub fn find_all(game: &Game, conn: &mut Connection) -> Result<Vec<ScoreDto>, Error> {
        let levels = Level::find_by_game(game, conn)?;
        let scores = Score::belonging_to(&levels)
            .select(Score::as_select())
            .load(conn)?
            .into_iter()
            .map(|s| ScoreDto::new(s, conn))
            .collect::<Vec<ScoreDto>>();

        Ok(scores)
    }

    /// Fetches a score from the database with the given id.
    /// 
    /// # Errors
    /// - If no score is found with the given id.
    pub fn find_by_id(score_id: Uuid, conn: &mut Connection) -> Result<ScoreDto, Error> {
        let score_data = score.find(score_id).get_result::<Score>(conn)?;

        Ok(ScoreDto::new(score_data, conn))
    }

    /// Fetchets all the scores in the database related to the given level. 
    pub fn find_by_level(
        level: &Level,
        include_hidden: bool,
        conn: &mut Connection,
    ) -> Result<Vec<ScoreDto>, Error> {
        let mut query = Score::belonging_to(level).into_boxed();

        if !include_hidden {
            query = query.filter(is_hidden.eq(false));
        }

        let scores = query
            .select(Score::as_select())
            .load::<Score>(conn)?
            .into_iter()
            .map(|s| ScoreDto::new(s, conn))
            .collect::<Vec<ScoreDto>>();

        Ok(scores)
    }

    /// Fetchets all the scores in the database related to the given user. 
    pub fn find_by_user(
        user: &User,
        include_hidden: bool,
        conn: &mut Connection,
    ) -> Result<Vec<ScoreDto>, Error> {
        let mut query = Score::belonging_to(user).into_boxed();

        if !include_hidden {
            query = query.filter(is_hidden.eq(false));
        }

        let scores = query
            .select(Score::as_select())
            .load::<Score>(conn)?
            .into_iter()
            .map(|s| ScoreDto::new(s, conn))
            .collect::<Vec<ScoreDto>>();

        Ok(scores)
    }

    /// Adds a new score to the database.
    /// 
    /// Errors
    /// - If one of the fields contain invalid data.
    pub fn insert(new_score: ScoreForm, conn: &mut Connection) -> Result<ScoreDto, Error> {
        let new_score = diesel::insert_into(score)
            .values(&new_score)
            .get_result::<Score>(conn)?;

        Ok(ScoreDto::new(new_score, conn))
    }

    /// Updates a score with the given id in the database.
    /// 
    /// Errors
    /// - If no score is found with the given id.
    /// - If one of the fields contain invalid data.
    pub fn update(
        score_id: Uuid,
        updated_score: ScoreForm,
        conn: &mut Connection,
    ) -> Result<ScoreDto, Error> {
        let updated_score = diesel::update(score)
            .filter(id.eq(score_id))
            .set(updated_score)
            .get_result::<Score>(conn)?;

        Ok(ScoreDto::new(updated_score, conn))
    }

    /// Deletes multiple scores from the database with the given ids.
    pub fn delete_many(score_ids: Vec<Uuid>, conn: &mut Connection) -> QueryResult<usize> {
        diesel::delete(score)
            .filter(id.eq_any(score_ids))
            .execute(conn)
    }

    /// Counts the number of scores in the database. If a game is given, only the number of scores relates to the game are
    /// counted.
    pub fn count(game: &Option<Game>, conn: &mut Connection) -> Result<i64, Error> {
        let count = if let Some(value) = game {
            let levels = Level::belonging_to(value)
                .select(Level::as_select())
                .load(conn)?;

            Score::belonging_to(&levels)
                .select(count_star())
                .first(conn)?
        } else {
            score.select(count_star()).first(conn)?
        };

        Ok(count)
    }
}

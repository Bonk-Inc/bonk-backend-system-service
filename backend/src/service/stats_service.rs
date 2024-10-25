use reqwest::StatusCode;
use uuid::Uuid;

use crate::{
    config::db::Pool,
    error::internal_error,
    models::{game, score},
};

pub fn count_games(pool: &Pool) -> Result<i64, (StatusCode, String)> {
    match game::count_games(&mut pool.get().unwrap()) {
        Ok(count) => Ok(count),
        Err(_) => Err(internal_error("Cannot count games in database".to_string())),
    }
}

pub fn count_scores(id: Option<Uuid>, pool: &Pool) -> Result<i64, (StatusCode, String)> {
    match score::count_score(id, &mut pool.get().unwrap()) {
        Ok(count) => Ok(count),
        Err(_) => Err(internal_error(
            "Cannot count scores in database".to_string(),
        )),
    }
}

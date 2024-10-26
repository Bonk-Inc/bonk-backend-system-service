use std::str::FromStr;

use axum::{http::StatusCode, Json};
use uuid::Uuid;

use crate::{
    config::db::Pool,
    error::{internal_error, not_found_error, ErrorResponse},
    models::{
        score::{self, Score, ScoreDTO},
        Model,
    },
};

use super::{game_service, level_service};

pub fn find_all(pool: &Pool) -> Result<Vec<Score>, (StatusCode, Json<ErrorResponse>)> {
    match Score::find_all(&mut pool.get().unwrap()) {
        Ok(scores) => Ok(scores),
        Err(_) => Err(internal_error(
            "Error while fetching scores occured".to_string(),
        )),
    }
}

pub fn find_by_id(id: Uuid, pool: &Pool) -> Result<Score, (StatusCode, Json<ErrorResponse>)> {
    match Score::find_by_id(id, &mut pool.get().unwrap()) {
        Ok(score) => Ok(score),
        Err(_) => Err(not_found_error(format!(
            "Score with id '{}' not found",
            id.to_string()
        ))),
    }
}

pub fn find_by_game(
    game_id: Uuid,
    include_hidden: bool,
    pool: &Pool,
) -> Result<Vec<Score>, (StatusCode, Json<ErrorResponse>)> {
    if !game_service::game_exisits(game_id, pool) {
        return Err(not_found_error(format!(
            "Game with id '{}' not found",
            game_id.to_string()
        )));
    }

    match score::find_by_game(game_id, include_hidden, &mut pool.get().unwrap()) {
        Ok(score) => Ok(score),
        Err(_) => Err(internal_error(
            "An error occured when trying to fetch scores".to_string(),
        )),
    }
}

pub fn find_by_level(
    level_id: Uuid,
    include_hidden: bool,
    pool: &Pool,
) -> Result<Vec<Score>, (StatusCode, Json<ErrorResponse>)> {
    if !level_service::level_exists(level_id, pool) {
        return Err(not_found_error(format!(
            "Level with id '{}' not found",
            level_id.to_string()
        )));
    }

    match score::find_by_level(level_id, include_hidden, &mut pool.get().unwrap()) {
        Ok(score) => Ok(score),
        Err(_) => Err(internal_error(
            "An error occured when trying to fetch scores".to_string(),
        )),
    }
}

pub fn insert(new_score: ScoreDTO, pool: &Pool) -> Result<Score, (StatusCode, Json<ErrorResponse>)> {
    match Score::insert(new_score, &mut pool.get().unwrap()) {
        Ok(score) => Ok(score),
        Err(err) => Err(internal_error(format!("Error saving new score, {}", err))),
    }
}

pub fn update(
    id: Uuid,
    updated_score: ScoreDTO,
    pool: &Pool,
) -> Result<Score, (StatusCode, Json<ErrorResponse>)> {
    if !score_exisits(id, pool) {
        return Err(not_found_error(format!(
            "Score with id '{}' not found",
            id.to_string()
        )));
    }

    match Score::update(id, updated_score, &mut pool.get().unwrap()) {
        Ok(score) => Ok(score),
        Err(_) => Err(internal_error("Error while updating score".to_string())),
    }
}

pub fn delete(ids: String, pool: &Pool) -> Result<usize, (StatusCode, Json<ErrorResponse>)> {
    let score_ids = ids
        .split(',')
        .filter_map(|s| Uuid::from_str(s).ok())
        .collect::<Vec<Uuid>>();

    match Score::delete_many(score_ids, &mut pool.get().unwrap()) {
        Ok(result) => Ok(result),
        Err(_) => Err(internal_error("Error while deleting score".to_string())),
    }
}

pub fn score_exisits(id: Uuid, pool: &Pool) -> bool {
    let score = Score::find_by_id(id, &mut pool.get().unwrap());

    return score.is_ok();
}

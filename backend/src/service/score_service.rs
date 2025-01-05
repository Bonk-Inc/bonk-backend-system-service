use std::str::FromStr;

use uuid::Uuid;

use crate::{
    config::db::Pool,
    error::{internal_error, not_found_error, ErrorResponse},
    models::score::{Score, ScoreDto, ScoreForm},
};

use super::{game_service, level_service, user_service};

/// Queries the database and fetches all the registerd scores from a game.
/// 
/// # Errors
/// 
/// This function fails if:
/// - could not find game with given id.
/// - an error occured during execution.
/// 
pub fn find_all(game_id: Uuid, pool: &Pool) -> Result<Vec<ScoreDto>, ErrorResponse> {
    let game: Result<crate::models::game::Game, ErrorResponse> = game_service::find_by_id(game_id, pool);
    if game.is_err() {
        return Err(not_found_error(format!(
            "Game with id '{}' not found",
            game_id.to_string()
        )));
    }

    match Score::find_all(&game.unwrap(), &mut pool.get().unwrap()) {
        Ok(scores) => Ok(scores),
        Err(_) => Err(internal_error(
            "Error while fetching scores occured".to_string(),
        )),
    }
}

/// Queries the database and fetches the registerd score by the given id.
/// 
/// # Errors
/// 
/// This function fails if:
/// - could not find score with given id.
/// 
pub fn find_by_id(id: Uuid, pool: &Pool) -> Result<ScoreDto, ErrorResponse> {
    match Score::find_by_id(id, &mut pool.get().unwrap()) {
        Ok(score) => Ok(score),
        Err(_) => Err(not_found_error(format!(
            "Score with id '{}' not found",
            id.to_string()
        ))),
    }
}

/// Queries the database and fetches the registerd scores by the given level.
/// 
/// # Errors
/// 
/// This function fails if:
/// - could not find level with given id.
/// - an error occured during execution.
/// 
pub fn find_by_level(
    level_id: Uuid,
    include_hidden: bool,
    pool: &Pool,
) -> Result<Vec<ScoreDto>, ErrorResponse> {
    let level = level_service::find_by_id(level_id, pool);
    if level.is_err() {
        return Err(not_found_error(format!(
            "Level with id '{}' not found",
            level_id.to_string()
        )));
    }

    match Score::find_by_level(&level.unwrap(), include_hidden, &mut pool.get().unwrap()) {
        Ok(score) => Ok(score),
        Err(_) => Err(internal_error(
            "An error occured when trying to fetch scores".to_string(),
        )),
    }
}

/// Queries the database and fetches the registerd scores by the given user.
/// 
/// # Errors
/// 
/// This function fails if:
/// - could not find user with given id.
/// - an error occured during execution.
/// 
pub fn find_by_user(
    user_id: Uuid,
    include_hidden: bool,
    pool: &Pool,
) -> Result<Vec<ScoreDto>, ErrorResponse> {
    let user = user_service::find_by_id(user_id, pool);
    if user.is_err() {
        return Err(not_found_error(format!(
            "User with id '{}' not found",
            user_id.to_string()
        )));
    }

    match Score::find_by_user(&user.unwrap(), include_hidden, &mut pool.get().unwrap()) {
        Ok(score) => Ok(score),
        Err(_) => Err(internal_error(
            "An error occured when trying to fetch scores".to_string(),
        )),
    }
}

/// Inserts a new score object and into the database.
/// 
/// # Errors
/// 
/// This function fails if:
/// - an error occured during execution.
/// 
pub fn insert(new_score: ScoreForm, pool: &Pool) -> Result<ScoreDto, ErrorResponse> {
    match Score::insert(new_score, &mut pool.get().unwrap()) {
        Ok(score) => Ok(score),
        Err(err) => Err(internal_error(format!("Error saving new score, {}", err))),
    }
}

/// Updates the score with the given id in the database.
/// 
/// # Errors
/// 
/// This function fails if:
/// - an error occured during execution.
/// - no score could be find with the given id.
/// 
pub fn update(
    id: Uuid,
    updated_score: ScoreForm,
    pool: &Pool,
) -> Result<ScoreDto, ErrorResponse> {
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

/// Deletes a score from the database with the given id.
/// 
/// # Errors
/// 
/// This function fails if:
/// - an error occured during execution.
/// - no score could be found with the given id.
/// 
pub fn delete(ids: String, pool: &Pool) -> Result<usize, ErrorResponse> {
    let score_ids = ids
        .split(',')
        .filter_map(|s| Uuid::from_str(s).ok())
        .collect::<Vec<Uuid>>();

    match Score::delete_many(score_ids, &mut pool.get().unwrap()) {
        Ok(result) => Ok(result),
        Err(_) => Err(internal_error("Error while deleting score".to_string())),
    }
}

/// Checks if a score exists in the database with the given id.
pub fn score_exisits(id: Uuid, pool: &Pool) -> bool {
    let score = Score::find_by_id(id, &mut pool.get().unwrap());

    return score.is_ok();
}

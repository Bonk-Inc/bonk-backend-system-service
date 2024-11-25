use uuid::Uuid;

use crate::{
    config::db::Pool,
    error::{internal_error, not_found_error, ErrorResponse},
    models::{
        level::{self, Level, LevelDTO},
        Model,
    },
};

use super::game_service;

/// Queries the database and fetches all the registerd levels.
/// 
/// # Errors
/// 
/// This function fails if:
/// - an error occured during execution.
/// 
pub fn find_all(pool: &Pool) -> Result<Vec<Level>, ErrorResponse> {
    match Level::find_all(&mut pool.get().unwrap()) {
        Ok(levels) => Ok(levels),
        Err(_) => Err(internal_error("Cannot fetch levels".to_string())),
    }
}

/// Queries the database and fetches the registerd level by the given id.
/// 
/// # Errors
/// 
/// This function fails if:
/// - an error occured during execution.
/// - could not find level with given id.
/// 
pub fn find_by_id(id: Uuid, pool: &Pool) -> Result<Level, ErrorResponse> {
    match Level::find_by_id(id, &mut pool.get().unwrap()) {
        Ok(level) => Ok(level),
        Err(_) => Err(not_found_error(format!(
            "Level with id '{}' not found",
            id.to_string()
        ))),
    }
}

/// Queries the database and fetches the registerd levels by the given game.
/// 
/// # Errors
/// 
/// This function fails if:
/// - could not find game with given id.
/// - an error occured during execution.
/// 
pub fn find_by_game(game_id: Uuid, pool: &Pool) -> Result<Vec<Level>, ErrorResponse> {
    if !game_service::game_exisits(game_id, pool) {
        return Err(not_found_error(format!(
            "Game with id '{}' not found",
            game_id.to_string()
        )));
    }

    match level::find_by_game(game_id, &mut pool.get().unwrap()) {
        Ok(levels) => Ok(levels),
        Err(_) => Err(internal_error(
            "Cannot add a new level in database".to_string(),
        )),
    }
}

/// Inserts a new level object and into the database.
/// 
/// # Errors
/// 
/// This function fails if:
/// - an error occured during execution.
/// 
pub fn insert(new_level: LevelDTO, pool: &Pool) -> Result<Level, ErrorResponse> {
    match Level::insert(new_level, &mut pool.get().unwrap()) {
        Ok(level) => Ok(level),
        Err(_) => Err(internal_error(
            "Cannot add a new level in database".to_string(),
        )),
    }
}

/// Updates the level with the given id in the database.
/// 
/// # Errors
/// 
/// This function fails if:
/// - an error occured during execution.
/// - no level could be find with the given id.
/// 
pub fn update(
    id: Uuid,
    updated_level: LevelDTO,
    pool: &Pool,
) -> Result<Level, ErrorResponse> {
    if !level_exists(id, pool) {
        return Err(not_found_error(format!(
            "Level with id '{}' not found",
            id.to_string()
        )));
    }

    match Level::update(id, updated_level, &mut pool.get().unwrap()) {
        Ok(level) => Ok(level),
        Err(_) => Err(internal_error("Could not update level".to_string())),
    }
}

/// Deletes a level from the database with the given id.
/// 
/// # Errors
/// 
/// This function fails if:
/// - an error occured during execution.
/// - no level could be found with the given id.
/// 
pub fn delete(id: Uuid, pool: &Pool) -> Result<usize, ErrorResponse> {
    if !level_exists(id, pool) {
        return Err(not_found_error(format!(
            "Level with id '{}' not found",
            id.to_string()
        )));
    }

    match Level::delete(id, &mut pool.get().unwrap()) {
        Ok(results) => Ok(results),
        Err(_) => Err(internal_error("Could not delete level".to_string())),
    }
}

/// Checks if a level exists in the database with the given id.
pub fn level_exists(id: Uuid, pool: &Pool) -> bool {
    Level::find_by_id(id, &mut pool.get().unwrap()).is_ok()
}

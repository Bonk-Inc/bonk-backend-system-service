use axum::http::StatusCode;
use uuid::Uuid;

use crate::{
    config::db::Pool,
    error::{internal_error, not_found_error},
    models::{
        level::{self, Level, LevelDTO},
        Model,
    },
};

use super::game_service;

pub fn find_all(pool: &Pool) -> Result<Vec<Level>, (StatusCode, String)> {
    match Level::find_all(&mut pool.get().unwrap()) {
        Ok(levels) => Ok(levels),
        Err(_) => Err(internal_error("Cannot fetch levels".to_string())),
    }
}

pub fn find_by_id(id: Uuid, pool: &Pool) -> Result<Level, (StatusCode, String)> {
    match Level::find_by_id(id, &mut pool.get().unwrap()) {
        Ok(level) => Ok(level),
        Err(_) => Err(not_found_error(format!(
            "Level with id '{}' not found",
            id.to_string()
        ))),
    }
}

pub fn find_by_game(game_id: Uuid, pool: &Pool) -> Result<Vec<Level>, (StatusCode, String)> {
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

pub fn insert(new_level: LevelDTO, pool: &Pool) -> Result<Level, (StatusCode, String)> {
    match Level::insert(new_level, &mut pool.get().unwrap()) {
        Ok(level) => Ok(level),
        Err(_) => Err(internal_error(
            "Cannot add a new level in database".to_string(),
        )),
    }
}

pub fn update(
    id: Uuid,
    updated_level: LevelDTO,
    pool: &Pool,
) -> Result<Level, (StatusCode, String)> {
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

pub fn delete(id: Uuid, pool: &Pool) -> Result<usize, (StatusCode, String)> {
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

pub fn level_exists(id: Uuid, pool: &Pool) -> bool {
    Level::find_by_id(id, &mut pool.get().unwrap()).is_ok()
}

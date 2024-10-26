use axum::{http::StatusCode, Json};
use uuid::Uuid;

use crate::{
    config::db::Pool, error::{internal_error, not_found_error, ErrorResponse}, models::{
        game::{Game, GameDTO},
        level::{Level, LevelDTO},
        Model,
    }
};

pub fn find_all(pool: &Pool) -> Result<Vec<Game>, (StatusCode, Json<ErrorResponse>)> {
    match Game::find_all(&mut pool.get().unwrap()) {
        Ok(games) => Ok(games),
        Err(_) => Err(internal_error("Cannot fetch games".to_string())),
    }
}

pub fn find_by_id(id: Uuid, pool: &Pool) -> Result<Game, (StatusCode, Json<ErrorResponse>)> {
    match Game::find_by_id(id, &mut pool.get().unwrap()) {
        Ok(game) => Ok(game),
        Err(_) => Err(not_found_error(format!("Game with id '{}' not found", id.to_string()),)),
    }
}

pub fn insert(new_game: GameDTO, pool: &Pool) -> Result<Game, (StatusCode, Json<ErrorResponse>)> {
    match Game::insert(new_game, &mut pool.get().unwrap()) {
        Ok(game) => {
            let level = LevelDTO {
                name: "Level 1".to_owned(),
                game_id: game.id,
            };

            match Level::insert(level, &mut pool.get().unwrap()) {
                Ok(_) => Ok(game),
                Err(_) => Err(internal_error("Could not add level to newly created game".to_string())),
            }
        }
        Err(_) => Err(internal_error("Could not add new game in database".to_string())),
    }
}

pub fn update(
    id: Uuid,
    updated_game: GameDTO,
    pool: &Pool,
) -> Result<Game, (StatusCode, Json<ErrorResponse>)> {
    if !game_exisits(id, pool) {
        return Err(not_found_error(format!("Game with id '{}' not found", id.to_string())));
    }

    match Game::update(id, updated_game, &mut pool.get().unwrap()) {
        Ok(game) => Ok(game),
        Err(_) => Err(internal_error("Could not update game".to_string())),
    }
}

pub fn delete(id: Uuid, pool: &Pool) -> Result<usize, (StatusCode, Json<ErrorResponse>)> {
    if !game_exisits(id, pool) {
        return Err(not_found_error(format!("Game with id '{}' not found", id.to_string())));
    }

    match Game::delete(id, &mut pool.get().unwrap()) {
        Ok(result) => Ok(result),
        Err(_) => Err(internal_error("Error occured when deleting game".to_string())),
    }
}

pub fn game_exisits(id: Uuid, pool: &Pool) -> bool {
    let game = Game::find_by_id(id, &mut pool.get().unwrap());

    game.is_ok()
}

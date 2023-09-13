use actix_web::web;
use uuid::Uuid;

use crate::{
    config::db::Pool,
    error::ServiceError,
    models::game::{Game, GameDTO},
};

pub fn find_all(pool: &web::Data<Pool>) -> Result<Vec<Game>, ServiceError> {
    match Game::find_all(&mut pool.get().unwrap()) {
        Ok(games) => Ok(games),
        Err(_) => Err(ServiceError::InternalServerError {
            error_message: "Cannot fetch games".to_string(),
        }),
    }
}

pub fn find_by_id(id: Uuid, pool: &web::Data<Pool>) -> Result<Game, ServiceError> {
    match Game::find_by_id(id, &mut pool.get().unwrap()) {
        Ok(game) => Ok(game),
        Err(_) => Err(ServiceError::NotFound {
            error_message: format!("Game with id '{}' not found", id.to_string()),
        }),
    }
}

pub fn insert(new_game: GameDTO, pool: &web::Data<Pool>) -> Result<Game, ServiceError> {
    match Game::insert(new_game, &mut pool.get().unwrap()) {
        Ok(game) => Ok(game),
        Err(_) => Err(ServiceError::InternalServerError {
            error_message: "Could not add new game in database".to_string(),
        }),
    }
}

pub fn update(
    id: Uuid,
    updated_game: GameDTO,
    pool: &web::Data<Pool>,
) -> Result<Game, ServiceError> {
    if !game_exisits(id, pool) {
        return Err(ServiceError::NotFound {
            error_message: format!("Game with id '{}' not found", id.to_string()),
        });
    }

    match Game::update(id, updated_game, &mut pool.get().unwrap()) {
        Ok(game) => Ok(game),
        Err(_) => Err(ServiceError::InternalServerError {
            error_message: "Could not update game".to_string(),
        }),
    }
}

pub fn delete(id: Uuid, pool: &web::Data<Pool>) -> Result<usize, ServiceError> {
    if !game_exisits(id, pool) {
        return Err(ServiceError::NotFound {
            error_message: format!("Game with id '{}' not found", id.to_string()),
        });
    }

    match Game::delete(id, &mut pool.get().unwrap()) {
        Ok(result) => Ok(result),
        Err(_) => Err(ServiceError::InternalServerError {
            error_message: "Error occured when deleting game".to_string(),
        }),
    }
}

pub fn game_exisits(id: Uuid, pool: &web::Data<Pool>) -> bool {
    let game = Game::find_by_id(id, &mut pool.get().unwrap());

    return game.is_ok();
}

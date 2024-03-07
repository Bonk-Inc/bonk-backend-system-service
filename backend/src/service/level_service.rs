use actix_web::web;
use babs::models::Level;
use uuid::Uuid;

use crate::{
    config::db::Pool,
    error::ServiceError,
    models::{level::{self, LevelDTO}, Model},
};

use super::game_service;

pub fn find_all(pool: &web::Data<Pool>) -> Result<Vec<Level>, ServiceError> {
    match Level::find_all(&mut pool.get().unwrap()) {
        Ok(levels) => Ok(levels),
        Err(_) => Err(ServiceError::InternalServerError {
            error_message: "Cannot fetch levels".to_string(),
        }),
    }
}

pub fn find_by_id(id: Uuid, pool: &web::Data<Pool>) -> Result<Level, ServiceError> {
    match Level::find_by_id(id, &mut pool.get().unwrap()) {
        Ok(level) => Ok(level),
        Err(_) => Err(ServiceError::NotFound {
            error_message: format!("Level with id '{}' not found", id.to_string()),
        }),
    }
}

pub fn find_by_game(game_id: Uuid, pool: &web::Data<Pool>) -> Result<Vec<Level>, ServiceError> {
    if !game_service::game_exisits(game_id, pool) {
        return Err(ServiceError::NotFound {
            error_message: format!("Game with id '{}' not found", game_id.to_string()),
        });
    }

    match level::find_by_game(game_id, &mut pool.get().unwrap()) {
        Ok(levels) => Ok(levels),
        Err(_) =>  Err(ServiceError::InternalServerError {
            error_message: "Cannot add a new level in database".to_string(),
        }),
    }
}

pub fn insert(new_level: LevelDTO, pool: &web::Data<Pool>) -> Result<Level, ServiceError> {
    match Level::insert(new_level, &mut pool.get().unwrap()) {
        Ok(level) => Ok(level),
        Err(_) => Err(ServiceError::InternalServerError {
            error_message: "Cannot add a new level in database".to_string(),
        }),
    }
}

pub fn update(
    id: Uuid,
    updated_level: LevelDTO,
    pool: &web::Data<Pool>,
) -> Result<Level, ServiceError> {
    if !level_exists(id, pool) {
        return Err(ServiceError::NotFound {
            error_message: format!("Level with id '{}' not found", id.to_string()),
        });
    }

    match Level::update(id, updated_level, &mut pool.get().unwrap()) {
        Ok(level) => Ok(level),
        Err(_) => Err(ServiceError::InternalServerError { 
            error_message: "Could not update level".to_string()
        })
    }
}

pub fn delete(id: Uuid, pool: &web::Data<Pool>) -> Result<usize, ServiceError> {
    if !level_exists(id, pool) {
        return Err(ServiceError::NotFound {
            error_message: format!("Level with id '{}' not found", id.to_string()),
        });
    }

    match Level::delete(id, &mut pool.get().unwrap()) {
        Ok(results) => Ok(results),
        Err(_) => Err(ServiceError::InternalServerError { 
            error_message: "Could not delete level".to_string()
        }),
    }
}

pub fn level_exists(id: Uuid, pool: &web::Data<Pool>) -> bool {
    Level::find_by_id(id, &mut pool.get().unwrap()).is_ok()
}
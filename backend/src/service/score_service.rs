use actix_web::web;
use uuid::Uuid;

use crate::{
    config::db::Pool,
    error::ServiceError,
    models::score::{Score, ScoreDTO},
};

use super::game_service;

pub fn find_all(pool: &web::Data<Pool>) -> Result<Vec<Score>, ServiceError> {
    match Score::find_all(&mut pool.get().unwrap()) {
        Ok(scores) => Ok(scores),
        Err(_) => Err(ServiceError::InternalServerError {
            error_message: "Error while fetching scores occured".to_string(),
        }),
    }
}

pub fn find_by_id(id: Uuid, pool: &web::Data<Pool>) -> Result<Score, ServiceError> {
    match Score::find_by_id(id, &mut pool.get().unwrap()) {
        Ok(score) => Ok(score),
        Err(_) => Err(ServiceError::NotFound {
            error_message: format!("Score with id '{}' not found", id.to_string()),
        }),
    }
}

pub fn find_by_game(
    game_id: Uuid,
    include_hidden: bool,
    pool: &web::Data<Pool>,
) -> Result<Vec<Score>, ServiceError> {
    if !game_service::game_exisits(game_id, pool) {
        return Err(ServiceError::NotFound {
            error_message: format!("Game with id '{}' not found", game_id.to_string()),
        });
    }

    match Score::find_by_game(game_id, include_hidden, &mut pool.get().unwrap()) {
        Ok(score) => Ok(score),
        Err(_) => Err(ServiceError::InternalServerError {
            error_message: "An error occured when trying to fetch scores".to_string(),
        }),
    }
}

pub fn insert(new_score: ScoreDTO, pool: &web::Data<Pool>) -> Result<Score, ServiceError> {
    match Score::insert(new_score, &mut pool.get().unwrap()) {
        Ok(score) => Ok(score),
        Err(_) => Err(ServiceError::NotFound {
            error_message: "Error saving new score".to_string(),
        }),
    }
}

pub fn update(
    id: Uuid,
    updated_score: ScoreDTO,
    pool: &web::Data<Pool>,
) -> Result<Score, ServiceError> {
    if !score_exisits(id, pool) {
        return Err(ServiceError::NotFound {
            error_message: format!("Score with id '{}' not found", id.to_string()),
        });
    }

    match Score::update(id, updated_score, &mut pool.get().unwrap()) {
        Ok(score) => Ok(score),
        Err(_) => Err(ServiceError::InternalServerError {
            error_message: "Error while updating score".to_string(),
        }),
    }
}

pub fn delete(id: Uuid, pool: &web::Data<Pool>) -> Result<usize, ServiceError> {
    if !score_exisits(id, pool) {
        return Err(ServiceError::NotFound {
            error_message: format!("Score with id '{}' not found", id.to_string()),
        });
    }

    match Score::delete(id, &mut pool.get().unwrap()) {
        Ok(result) => Ok(result),
        Err(_) => Err(ServiceError::InternalServerError {
            error_message: "Error while deleting score".to_string(),
        }),
    }
}

pub fn score_exisits(id: Uuid, pool: &web::Data<Pool>) -> bool {
    let score = Score::find_by_id(id, &mut pool.get().unwrap());

    return score.is_ok();
}

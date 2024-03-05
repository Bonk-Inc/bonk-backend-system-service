use actix_web::web;
use babs::models::Level;

use crate::{config::db::Pool, error::ServiceError, models::{level::LevelDTO, Model}};

pub fn find_all(pool: &web::Data<Pool>) -> Result<Vec<Level>, ServiceError> {
    match Level::find_all(&mut pool.get().unwrap()) {
        Ok(levels) => Ok(levels),
        Err(_) => Err(ServiceError::InternalServerError {
            error_message: "Cannot fetch levels".to_string(),
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
use actix_web::web;

use crate::{
    error::ServiceError,
    models::{game, score}, config::db::Pool
};

pub fn count_games(pool: &web::Data<Pool>) -> Result<i64, ServiceError> {
    match game::count_games(&mut pool.get().unwrap()) {
        Ok(count) => Ok(count),
        Err(_) => Err(ServiceError::InternalServerError { 
            error_message: "Cannot count games in database".to_string()
        }),
    }
}

pub fn count_scores(pool: &web::Data<Pool>) -> Result<i64, ServiceError> {
    match score::count_score(&mut pool.get().unwrap()) {
        Ok(count) => Ok(count),
        Err(_) => Err(ServiceError::InternalServerError { 
            error_message: "Cannot count scores in database".to_string()
        }),
    }
}

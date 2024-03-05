use actix_web::web;
use babs::models::Level;

use crate::{config::db::Pool, error::ServiceError, models::Model};

pub fn find_all(pool: &web::Data<Pool>) -> Result<Vec<Level>, ServiceError> {
    match Level::find_all(&mut pool.get().unwrap()) {
        Ok(levels) => Ok(levels),
        Err(_) => Err(ServiceError::InternalServerError {
            error_message: "Cannot fetch games".to_string(),
        }),
    }
}
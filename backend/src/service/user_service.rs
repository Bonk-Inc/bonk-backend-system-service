use uuid::Uuid;

use crate::{
    config::db::Pool,
    error::{internal_error, not_found_error, ErrorResponse},
    models::{user::{self, User, UserDTO}, Model},
};

use super::game_service;

/// Queries the database and fetches the registerd users in a game.
/// 
/// # Errors
/// 
/// This function fails if:
/// - an error occured during execution.
/// - no game could be found with the given id.
/// 
pub fn find_by_game(game_id: Uuid, pool: &Pool) -> Result<Vec<User>, ErrorResponse> {
    if !game_service::game_exisits(game_id, pool) {
        return Err(not_found_error(format!(
            "Game with id '{}' not found",
            game_id.to_string()
        )));
    }

    match user::find_by_game(game_id, &mut pool.get().unwrap()) {
        Ok(users) => Ok(users),
        Err(_) => Err(internal_error("Cannot fetch users".to_string())),
    }
}

/// Queries the database and fetches the registerd user with id.
/// 
/// # Errors
/// 
/// This function fails if:
/// - an error occured during execution.
/// - no user could be found with the given id.
/// 
pub fn find_by_id(id: Uuid, pool: &Pool) -> Result<User, ErrorResponse> {
    match User::find_by_id(id, &mut pool.get().unwrap()) {
        Ok(game) => Ok(game),
        Err(_) => Err(not_found_error(format!("User with id '{}' not found", id.to_string()),)),
    }
}

/// Inserts a new user into the database.
/// 
/// # Errors
/// 
/// This function fails if:
/// - an error occured during execution.
/// 
pub fn insert(new_user: UserDTO, pool: &Pool) -> Result<User, ErrorResponse> {
    match User::insert(new_user, &mut pool.get().unwrap()) {
        Ok(score) => Ok(score),
        Err(err) => Err(internal_error(format!("Error saving new user, {}", err))),
    }
}

/// Updates a user in the database with the given id.
/// 
/// # Errors
/// 
/// This function fails if:
/// - an error occured during execution.
/// - no user could be found with the given id.
/// 
pub fn update(
    id: Uuid,
    updated_level: UserDTO,
    pool: &Pool,
) -> Result<User, ErrorResponse> {
    if !user_exists(id, pool) {
        return Err(not_found_error(format!(
            "User with id '{}' not found",
            id.to_string()
        )));
    }

    match User::update(id, updated_level, &mut pool.get().unwrap()) {
        Ok(level) => Ok(level),
        Err(_) => Err(internal_error("Could not update user".to_string())),
    }
}

/// Deletes a user in the database with the given id.
/// 
/// # Errors
/// 
/// This function fails if:
/// - an error occured during execution.
/// - no user could be found with the given id.
/// 
pub fn delete(id: Uuid, pool: &Pool) -> Result<usize, ErrorResponse> {
    if !user_exists(id, pool) {
        return Err(not_found_error(format!(
            "User with id '{}' not found",
            id.to_string()
        )));
    }

    match User::delete(id, &mut pool.get().unwrap()) {
        Ok(results) => Ok(results),
        Err(_) => Err(internal_error("Could not delete user".to_string())),
    }
}

/// Checks if a user exists in the database with the given id.
pub fn user_exists(id: Uuid, pool: &Pool) -> bool {
    User::find_by_id(id, &mut pool.get().unwrap()).is_ok()
}

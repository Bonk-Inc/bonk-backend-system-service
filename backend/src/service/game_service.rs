use uuid::Uuid;

use crate::{
    config::db::Pool, error::{internal_error, not_found_error, ErrorResponse}, models::{
        game::{Game, GameDTO},
        level::{Level, LevelForm},
    }
};

/// Queries the database and fetches all the registerd games.
/// 
/// # Errors
/// 
/// This function fails if:
/// - an error occured during execution.
/// 
pub fn find_all(pool: &Pool) -> Result<Vec<Game>, ErrorResponse> {
    match Game::find_all(&mut pool.get().unwrap()) {
        Ok(games) => Ok(games),
        Err(_) => Err(internal_error("Cannot fetch games".to_string())),
    }
}

/// Queries the database and fetches the registerd game by the given id.
/// 
/// # Errors
/// 
/// This function fails if:
/// - an error occured during execution.
/// - could not find game with given id.
/// 
pub fn find_by_id(id: Uuid, pool: &Pool) -> Result<Game, ErrorResponse> {
    match Game::find_by_id(id, &mut pool.get().unwrap()) {
        Ok(game) => Ok(game),
        Err(_) => Err(not_found_error(format!("Game with id '{}' not found", id.to_string()),)),
    }
}

/// Inserts a new game object and into the database and adds a new level
/// to the game.
/// 
/// # Errors
/// 
/// This function fails if:
/// - an error occured during execution.
/// 
pub fn insert(new_game: GameDTO, pool: &Pool) -> Result<Game, ErrorResponse> {
    match Game::insert(new_game, &mut pool.get().unwrap()) {
        Ok(game) => {
            let level = LevelForm {
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

/// Updates the game with the given id in the database.
/// 
/// # Errors
/// 
/// This function fails if:
/// - an error occured during execution.
/// - no game could be find with the given id.
/// 
pub fn update(
    id: Uuid,
    updated_game: GameDTO,
    pool: &Pool,
) -> Result<Game, ErrorResponse> {
    if !game_exisits(id, pool) {
        return Err(not_found_error(format!("Game with id '{}' not found", id.to_string())));
    }

    match Game::update(id, updated_game, &mut pool.get().unwrap()) {
        Ok(game) => Ok(game),
        Err(_) => Err(internal_error("Could not update game".to_string())),
    }
}

/// Deletes a game from the database with the given id.
/// 
/// # Errors
/// 
/// This function fails if:
/// - an error occured during execution.
/// - no game could be found with the given id.
/// 
pub fn delete(id: Uuid, pool: &Pool) -> Result<usize, ErrorResponse> {
    if !game_exisits(id, pool) {
        return Err(not_found_error(format!("Game with id '{}' not found", id.to_string())));
    }

    match Game::delete(id, &mut pool.get().unwrap()) {
        Ok(result) => Ok(result),
        Err(_) => Err(internal_error("Error occured when deleting game".to_string())),
    }
}

/// Checks if a game exists in the database with the given id.
pub fn game_exisits(id: Uuid, pool: &Pool) -> bool {
    let game = Game::find_by_id(id, &mut pool.get().unwrap());

    game.is_ok()
}

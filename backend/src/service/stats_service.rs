use uuid::Uuid;

use crate::{
    config::db::Pool,
    error::{internal_error, ErrorResponse}, 
    models::{game::Game, score::Score}
};

/// Queries the database and counts the registerd games.
/// 
/// # Errors
/// 
/// This function fails if:
/// - an error occured during execution.
/// 
pub fn count_games(pool: &Pool) -> Result<i64, ErrorResponse> {
    match Game::count_games(&mut pool.get().unwrap()) {
        Ok(count) => Ok(count),
        Err(_) => Err(internal_error("Cannot count games in database".to_string())),
    }
}

/// Queries the database and counts the registerd score in a game.
/// 
/// # Errors
/// 
/// This function fails if:
/// - an error occured during execution.
/// 
pub fn count_scores(id: Option<Uuid>, pool: &Pool) -> Result<i64, ErrorResponse> {
    match Score::count_score(id, &mut pool.get().unwrap()) {
        Ok(count) => Ok(count),
        Err(_) => Err(internal_error(
            "Cannot count scores in database".to_string(),
        )),
    }
}

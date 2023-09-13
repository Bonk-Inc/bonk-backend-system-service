use actix_web::{Scope, web};

pub mod game;
pub mod score;

pub fn score_scope() -> Scope {
    web::scope("/score")
        .service(score::index)
        .service(score::show)
        .service(score::store)
        .service(score::game_scores)
        .service(score::update)
        .service(score::destroy)
}

pub fn game_scope() -> Scope {
    web::scope("/game")
        .service(game::index)
        .service(game::show)
        .service(game::store)
        .service(game::update)
        .service(game::destroy)
}
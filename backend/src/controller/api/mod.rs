use actix_web::{Scope, web};

pub mod game;
pub mod level;
pub mod score;
pub mod stats;

pub fn game_scope() -> Scope {
    web::scope("/game")
        .service(game::index)
        .service(game::show)
        .service(game::store)
        .service(game::update)
        .service(game::destroy)
}

pub fn level_scope() -> Scope {
    web::scope("/level")
        .service(level::index)
        .service(level::store)
        .service(level::update)
}

pub fn score_scope() -> Scope {
    web::scope("/score")
        .service(score::index)
        .service(score::show)
        .service(score::store)
        .service(score::game_scores)
        .service(score::update)
        .service(score::destroy)
}

pub fn stats_scope() -> Scope {
    web::scope("/stats")
        .service(stats::all)
        .service(stats::game_stats)
}
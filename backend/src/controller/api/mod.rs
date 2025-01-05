use axum::{routing::{get, post, put}, Router};

use crate::SharedState;

pub mod game;
pub mod level;
pub mod score;
pub mod stats;
pub mod user;

pub fn game_routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(game::index).post(game::store))
        .route("/{id}", get(game::show).put(game::update).delete(game::destroy))
}

pub fn level_routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(level::index).post(level::store))
        .route("/game/{id}", get(level::game_levels))
        .route("/{id}", put(level::update).delete(level::destroy))
}

pub fn score_routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(score::index).post(score::store))
        .route("/{id}", get(score::show).put(score::update).delete(score::destroy))
        .route("/game/{id}", get(score::game_scores))
        .route("/level/{id}", get(score::level_scores))
}

pub fn stats_routes() -> Router<SharedState> {
    Router::new()
        .route("/all", get(stats::all))
        .route("/game/{id}", get(stats::game_stats))
}

pub fn user_routes() -> Router<SharedState> {
    Router::new()
        .route("/", post(user::store))
        .route("/game/{id}", get(user::index))
        .route("/{id}", put(user::update).delete(user::destroy))
}

pub async fn healthcheck() -> &'static str {
    "Ok"
}
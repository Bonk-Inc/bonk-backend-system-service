use axum::{
    extract::{Path, State},
    Json
};
use uuid::Uuid;

use crate::{
    error::ErrorResponse, models::{
        respone::ResponseBody,
        stats::{GameStats, GlobalStats},
    }, service::stats_service, SharedState
};

pub async fn all(
    State(app_state): State<SharedState>,
) -> Result<Json<ResponseBody<GlobalStats>>, ErrorResponse> {
    let pool = &app_state.read().unwrap().db;
    let game_count = stats_service::count_games(&pool)?;
    let score_count = stats_service::count_scores(None, &pool)?;
    let user_count = stats_service::count_users(None, &pool)?;

    let stats = GlobalStats {
        games: game_count,
        scores: score_count,
        users: user_count
    };

    Ok(Json(ResponseBody::new("Global stats fechted", stats)))
}

pub async fn game_stats(
    Path(game_id): Path<Uuid>,
    State(app_state): State<SharedState>,
) -> Result<Json<ResponseBody<GameStats>>, ErrorResponse> {
    let pool = &app_state.read().unwrap().db;
    let score_count = stats_service::count_scores(Some(game_id), pool)?;
    let user_count = stats_service::count_users(Some(game_id), pool)?;

    let game_stats = GameStats {
        scores: score_count,
        users: user_count
    };

    Ok(Json(ResponseBody::new("Game stats fetched", game_stats)))
}

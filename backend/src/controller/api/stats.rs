use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{
    models::{
        respone::ResponseBody,
        stats::{GameStats, GlobalStats},
    },
    service::stats_service,
    SharedState,
};

pub async fn all(
    State(app_state): State<SharedState>,
) -> Result<Json<ResponseBody<GlobalStats>>, (StatusCode, String)> {
    let pool = &app_state.read().unwrap().db;
    let game_count = stats_service::count_games(&pool)?;
    let score_count = stats_service::count_scores(None, &pool)?;

    let stats = GlobalStats {
        games: game_count,
        scores: score_count,
    };

    Ok(Json(ResponseBody::new("Global stats fechted", stats)))
}

pub async fn game_stats(
    Path(id): Path<Uuid>,
    State(app_state): State<SharedState>,
) -> Result<Json<ResponseBody<GameStats>>, (StatusCode, String)> {
    let pool = &app_state.read().unwrap().db;
    let score_count = stats_service::count_scores(Some(id), pool)?;

    let game_stats = GameStats {
        scores: score_count,
    };

    Ok(Json(ResponseBody::new("Game stats fetched", game_stats)))
}

use actix_web::{HttpResponse, web, get};
use babs::{respone::ResponseBody, models::{GlobalStats, GameStats}};
use uuid::Uuid;

use crate::{error::ServiceError, service::stats_service, config::db::Pool};

#[get("/all/")]
pub async fn all(pool: web::Data<Pool>) -> actix_web::Result<HttpResponse, ServiceError> {
    let game_count = stats_service::count_games(&pool)?;
    let score_count = stats_service::count_scores(None, &pool)?;

    let stats = GlobalStats {
        games: game_count,
        scores: score_count
    };

    Ok(HttpResponse::Ok().json(ResponseBody::new("Stats fetched", stats)))
}

#[get("/game/{id}/")]
pub async fn game_stats(path: web::Path<Uuid>, pool: web::Data<Pool>) -> actix_web::Result<HttpResponse, ServiceError> {
    let score_count = stats_service::count_scores(Some(path.into_inner()), &pool)?;

    let game_stats = GameStats {
        scores: score_count,
    };

    Ok(HttpResponse::Ok().json(ResponseBody::new("Game stats fetched", game_stats)))
}
use actix_web::{HttpResponse, web, get};
use babs::{respone::ResponseBody, models::Stats};

use crate::{error::ServiceError, service::stats_service, config::db::Pool};

#[get("/all/")]
pub async fn all(pool: web::Data<Pool>) -> actix_web::Result<HttpResponse, ServiceError> {
    let game_count = stats_service::count_games(&pool)?;
    let score_count = stats_service::count_scores(&pool)?;

    let stats = Stats {
        games: game_count,
        scores: score_count
    };

    Ok(HttpResponse::Ok().json(ResponseBody::new("Stats fetched", stats)))
}
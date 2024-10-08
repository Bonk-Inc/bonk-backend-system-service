use actix_web::{delete, get, post, put, web, HttpResponse};
use serde::Deserialize;
use utoipa::{OpenApi, ToSchema};
use uuid::Uuid;

use crate::{
    config::db::Pool,
    error::ServiceError,
    models::{respone::ResponseBody, score::{Score, ScoreDTO}},
    service::score_service,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        index,
        show,
        game_scores,
        level_scores,
        store,
        update,
        destroy
    ),
    components(schemas(Score, ScoreDTO, ScoreResponseBody, ScoresResponseBody))
)]
pub(super) struct ScoreApi;

#[derive(ToSchema)]
pub struct ScoreResponseBody {
    pub message: String,
    pub data: Score,
}

#[derive(ToSchema)]
pub struct ScoresResponseBody {
    pub message: String,
    pub data: Vec<Score>,
}

#[derive(Deserialize)]
pub struct QueryParams {
    pub hidden: bool,
}

#[utoipa::path(
    get,
    path = "/score",
    tag = "Score",
    operation_id = "score_index",
    responses(
        (status = StatusCode::OK, description = "Scores fetched successfully", body = ScoresResponseBody)
    )
)]
#[get("/")]
pub async fn index(pool: web::Data<Pool>) -> actix_web::Result<HttpResponse, ServiceError> {
    match score_service::find_all(&pool) {
        Ok(scores) => Ok(HttpResponse::Ok().json(ResponseBody::new("Scores fetched", scores))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/score/{id}",
    tag = "Score",
    operation_id = "score_show",
    params(
        ("id", Path, description = "Unique id of a Score")
    ),
    responses(
        (status = StatusCode::OK, description = "Score fetched successfully", body = ScoreResponseBody),
        (status = StatusCode::NOT_FOUND, description = "No score found by id")
    )
)]
#[get("/{id}/")]
pub async fn show(
    pool: web::Data<Pool>,
    path: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse, ServiceError> {
    match score_service::find_by_id(path.into_inner(), &pool) {
        Ok(score) => Ok(HttpResponse::Ok().json(ResponseBody::new("Score fetched", score))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/score/game/{gameId}",
    tag = "Score",
    operation_id = "score_game_score",
    params(
        ("gameId", Path, description = "Unique id of a Game"),
        ("hidden", Query, description = "If hidden scores should also be fetched")
    ),
    responses(
        (status = StatusCode::OK, description = "Score fetched by game successfully", body = ScoresResponseBody),
        (status = StatusCode::NOT_FOUND, description = "No Game found by game id")
    )
)]
#[get("/game/{gameId}/")]
pub async fn game_scores(
    pool: web::Data<Pool>,
    path: web::Path<Uuid>,
    query: web::Query<QueryParams>,
) -> actix_web::Result<HttpResponse, ServiceError> {
    let query_params = query.into_inner();

    match score_service::find_by_game(path.into_inner(), query_params.hidden, &pool) {
        Ok(scores) => Ok(HttpResponse::Ok().json(ResponseBody::new("Scores fetched", scores))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/score/level/{levelId}",
    tag = "Score",
    operation_id = "score_level_score",
    params(
        ("levelId", Path, description = "Unique id of a Level"),
        ("hidden", Query, description = "If hidden scores should also be fetched")
    ),
    responses(
        (status = StatusCode::OK, description = "Score fetched by level successfully", body = ScoresResponseBody),
        (status = StatusCode::NOT_FOUND, description = "No Level found by level id")
    )
)]
#[get("/level/{levelId}/")]
pub async fn level_scores(
    pool: web::Data<Pool>,
    path: web::Path<Uuid>,
    query: web::Query<QueryParams>,
) -> actix_web::Result<HttpResponse, ServiceError> {
    let query_params = query.into_inner();

    match score_service::find_by_level(path.into_inner(), query_params.hidden, &pool) {
        Ok(scores) => Ok(HttpResponse::Ok().json(ResponseBody::new("Scores fetched", scores))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/score",
    tag = "Score",
    operation_id = "score_store",
    request_body = ScoreDTO,
    responses(
        (status = StatusCode::CREATED, description = "Score created successfully", body = ScoreResponseBody),
        (status = StatusCode::BAD_REQUEST, description = "Invalid input")
    )
)]
#[post("/")]
pub async fn store(
    pool: web::Data<Pool>,
    data: web::Json<ScoreDTO>,
) -> actix_web::Result<HttpResponse, ServiceError> {
    match score_service::insert(data.into_inner(), &pool) {
        Ok(scores) => Ok(HttpResponse::Created().json(ResponseBody::new("Score saved", scores))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    put,
    path = "/score/{id}",
    tag = "Score",
    operation_id = "score_update",
    request_body = ScoreDTO,
    params(
        ("id", Path, description = "Unique id of a Score")
    ),
    responses(
        (status = StatusCode::OK, description = "Score updated successfully", body = ScoreResponseBody),
        (status = StatusCode::BAD_REQUEST, description = "Invalid input"),
        (status = StatusCode::NOT_FOUND, description = "No score found by id")
    )
)]
#[put("/{id}/")]
pub async fn update(
    pool: web::Data<Pool>,
    data: web::Json<ScoreDTO>,
    path: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse, ServiceError> {
    match score_service::update(path.into_inner(), data.into_inner(), &pool) {
        Ok(scores) => Ok(HttpResponse::Ok().json(ResponseBody::new("Score updated", scores))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    delete,
    path = "/score/{id}",
    tag = "Score",
    operation_id = "score_destroy",
    params(
        ("id", Path, description = "Unique id(s) of a Score (splits on comma)")
    ),
    responses(
        (status = StatusCode::NO_CONTENT, description = "Score deleted successfully"),
        (status = StatusCode::NOT_FOUND, description = "No score found by id")
    )
)]
#[delete("/({id})/")]
pub async fn destroy(
    pool: web::Data<Pool>,
    path: web::Path<String>,
) -> actix_web::Result<HttpResponse, ServiceError> {
    match score_service::delete(path.into_inner(), &pool) {
        Ok(_) => Ok(HttpResponse::NoContent().body("")),
        Err(err) => Err(err),
    }
}

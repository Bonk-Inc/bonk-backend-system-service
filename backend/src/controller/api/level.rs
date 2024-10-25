use actix_web::{delete, get, post, put, web, HttpResponse};
use utoipa::{OpenApi, ToSchema};
use uuid::Uuid;

use crate::{
    config::db::Pool,
    error::ServiceError,
    models::{level::{Level, LevelDTO}, respone::ResponseBody},
    service::level_service,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        index,
        game_levels,
        store,
        update,
        destroy
    ),
    components(schemas(Level, LevelDTO, LevelResponseBody, LevelsResponseBody))
)]
pub struct LevelApi;

#[derive(ToSchema)]
pub struct LevelResponseBody {
    pub message: String,
    pub data: Level,
}

#[derive(ToSchema)]
pub struct LevelsResponseBody {
    pub message: String,
    pub data: Vec<Level>,
}

#[utoipa::path(
    get,
    path = "/",
    tag = "Level",
    operation_id = "level_index",
    responses(
        (status = StatusCode::OK, description = "Level fetched successfully", body = LevelsResponseBody)
    )
)]
#[get("/")]
pub async fn index(pool: web::Data<Pool>) -> actix_web::Result<HttpResponse, ServiceError> {
    match level_service::find_all(&pool) {
        Ok(levels) => Ok(HttpResponse::Ok().json(ResponseBody::new("Levels fetched", levels))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/game/{gameId}/",
    tag = "Level",
    operation_id = "level_games",
    params(
        ("gameId", Path, description = "Unique id of a Game"),
    ),
    responses(
        (status = StatusCode::OK, description = "Levels fetched by game successfully", body = LevelsResponseBody),
        (status = StatusCode::NOT_FOUND, description = "No Game found by game id")
    )
)]
#[get("/game/{gameId}/")]
pub async fn game_levels(
    pool: web::Data<Pool>,
    path: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse, ServiceError> {
    match level_service::find_by_game(path.into_inner(), &pool) {
        Ok(levels) => Ok(HttpResponse::Ok().json(ResponseBody::new("Levels fetched", levels))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/",
    tag = "Level",
    operation_id = "level_store",
    request_body = LevelDTO,
    responses(
        (status = StatusCode::CREATED, description = "Level created successfully", body = LevelsResponseBody),
        (status = StatusCode::BAD_REQUEST, description = "Invalid input")
    )
)]
#[post("/")]
pub async fn store(
    pool: web::Data<Pool>,
    data: web::Json<LevelDTO>,
) -> actix_web::Result<HttpResponse, ServiceError> {
    match level_service::insert(data.into_inner(), &pool) {
        Ok(level) => Ok(HttpResponse::Created().json(ResponseBody::new("Level created", level))),
        Err(error) => Err(error),
    }
}

#[utoipa::path(
    put,
    path = "/{id}/",
    tag = "Level",
    operation_id = "level_update",
    request_body = LevelDTO,
    params(
        ("id", Path, description = "Unique id of a Level"),
    ),
    responses(
        (status = StatusCode::OK, description = "Level updated successfully", body = LevelsResponseBody),
        (status = StatusCode::BAD_REQUEST, description = "Invalid input"),
        (status = StatusCode::NOT_FOUND, description = "No level found by id")
    )
)]
#[put("/{id}/")]
pub async fn update(
    pool: web::Data<Pool>,
    data: web::Json<LevelDTO>,
    path: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse, ServiceError> {
    match level_service::update(path.into_inner(), data.into_inner(), &pool) {
        Ok(level) => Ok(HttpResponse::Ok().json(ResponseBody::new("Level updated", level))),
        Err(error) => Err(error),
    }
}

#[utoipa::path(
    delete,
    path = "/{id}/",
    tag = "Level",
    operation_id = "level_destroy",
    params(
        ("id", Path, description = "Unique id of a Level"),
    ),
    responses(
        (status = StatusCode::NO_CONTENT, description = "Level deleted successfully"),
        (status = StatusCode::NOT_FOUND, description = "No level found by id")
    )
)]
#[delete("/{id}/")]
pub async fn destroy(
    pool: web::Data<Pool>,
    path: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse, ServiceError> {
    match level_service::delete(path.into_inner(), &pool) {
        Ok(_) => Ok(HttpResponse::NoContent().body("")),
        Err(err) => Err(err),
    }
}

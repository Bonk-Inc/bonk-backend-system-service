use axum::{extract::{State, Path}, http::StatusCode, Json};
use utoipa::{OpenApi, ToSchema};
use uuid::Uuid;

use crate::{
    error::ErrorResponse, models::{level::{Level, LevelDTO}, respone::ResponseBody}, service::level_service, SharedState
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
    path = "",
    tag = "Level",
    operation_id = "level_index",
    responses(
        (status = StatusCode::OK, description = "Level fetched successfully", body = LevelsResponseBody)
    )
)]
pub async fn index(
    State(app_state): State<SharedState>,
) -> Result<Json<ResponseBody<Vec<Level>>>, (StatusCode, Json<ErrorResponse>)> {
    let pool = &app_state.read().unwrap().db;

    match level_service::find_all(pool) {
        Ok(levels) => Ok(Json(ResponseBody::new("Levels fetched", levels))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/game/{gameId}",
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
pub async fn game_levels(
    State(app_state): State<SharedState>,
    Path(game_id): Path<Uuid>,
) -> Result<Json<ResponseBody<Vec<Level>>>, (StatusCode, Json<ErrorResponse>)> {
    let pool = &app_state.read().unwrap().db;

    match level_service::find_by_game(game_id, pool) {
        Ok(levels) => Ok(Json(ResponseBody::new("Levels fetched", levels))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "",
    tag = "Level",
    operation_id = "level_store",
    request_body = LevelDTO,
    responses(
        (status = StatusCode::CREATED, description = "Level created successfully", body = LevelsResponseBody),
        (status = StatusCode::BAD_REQUEST, description = "Invalid input")
    )
)]
pub async fn store(
    State(app_state): State<SharedState>,
    Json(new_level): Json<LevelDTO>,
) -> Result<(StatusCode, Json<ResponseBody<Level>>), (StatusCode, Json<ErrorResponse>)> {
    let pool = &app_state.read().unwrap().db;

    match level_service::insert(new_level, pool) {
        Ok(level) => Ok((StatusCode::CREATED, Json(ResponseBody::new("Level created", level)))),
        Err(error) => Err(error),
    }
}

#[utoipa::path(
    put,
    path = "/{id}",
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
pub async fn update(
    State(app_state): State<SharedState>,
    Path(id): Path<Uuid>,
    Json(updated_level): Json<LevelDTO>,
) -> Result<Json<ResponseBody<Level>>, (StatusCode, Json<ErrorResponse>)> {
    let pool = &app_state.read().unwrap().db;

    match level_service::update(id, updated_level, pool) {
        Ok(level) => Ok(Json(ResponseBody::new("Level updated", level))),
        Err(error) => Err(error),
    }
}

#[utoipa::path(
    delete,
    path = "/{id}",
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
pub async fn destroy(
    State(app_state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let pool = &app_state.read().unwrap().db;

    match level_service::delete(id, pool) {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => Err(err),
    }
}

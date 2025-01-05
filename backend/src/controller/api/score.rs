use std::collections::HashMap;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use utoipa::{OpenApi, ToSchema};
use uuid::Uuid;

use crate::{
    error::ErrorResponse,
    models::{
        respone::ResponseBody,
        score::{Score, ScoreDTO},
    },
    service::score_service,
    SharedState,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        index,
        show,
        level_scores,
        user_scores,
        store,
        update,
        destroy
    ),
    components(schemas(Score, ScoreDTO, ScoreResponseBody, ScoresResponseBody))
)]
pub struct ScoreApi;

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
    path = "",
    tag = "Score",
    operation_id = "score_index",
    responses(
        (status = StatusCode::OK, description = "Scores fetched successfully", body = ScoresResponseBody)
    )
)]
pub async fn index(
    State(app_state): State<SharedState>,
) -> Result<Json<ResponseBody<Vec<Score>>>, ErrorResponse> {
    let pool = &app_state.read().unwrap().db;

    match score_service::find_all(pool) {
        Ok(scores) => Ok(Json(ResponseBody::new("Scores fetched", scores))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/{id}",
    tag = "Score",
    operation_id = "score_show",
    params(
        ("id", Path, description = "Unique id of a Score")
    ),
    responses(
        (status = StatusCode::OK, description = "Score fetched successfully", body = ScoreResponseBody),
        (status = StatusCode::NOT_FOUND, description = "No score found by id", body = ErrorResponse)
    )
)]
pub async fn show(
    State(app_state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ResponseBody<Score>>, ErrorResponse> {
    let pool = &app_state.read().unwrap().db;

    match score_service::find_by_id(id, pool) {
        Ok(score) => Ok(Json(ResponseBody::new("Score fetched", score))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/level/{levelId}",
    tag = "Score",
    operation_id = "score_level_score",
    params(
        ("levelId", Path, description = "Unique id of a Level"),
        ("hidden", Query, description = "If hidden scores should also be fetched")
    ),
    responses(
        (status = StatusCode::OK, description = "Score fetched by level successfully", body = ScoresResponseBody),
        (status = StatusCode::NOT_FOUND, description = "No Level found by level id", body = ErrorResponse)
    )
)]
pub async fn level_scores(
    State(app_state): State<SharedState>,
    Path(level_id): Path<Uuid>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ResponseBody<Vec<Score>>>, ErrorResponse> {
    let pool = &app_state.read().unwrap().db;
    let show_hidden = params
        .get("hidden")
        .unwrap_or(&"false".to_string())
        .to_lowercase()
        .eq("true");

    match score_service::find_by_level(level_id, show_hidden, pool) {
        Ok(scores) => Ok(Json(ResponseBody::new("Scores fetched", scores))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/level/{userId}",
    tag = "Score",
    operation_id = "score_user_score",
    params(
        ("userId", Path, description = "Unique id of a User"),
        ("hidden", Query, description = "If hidden scores should also be fetched")
    ),
    responses(
        (status = StatusCode::OK, description = "Score fetched by user successfully", body = ScoresResponseBody),
        (status = StatusCode::NOT_FOUND, description = "No user found by user id", body = ErrorResponse)
    )
)]
pub async fn user_scores(
    State(app_state): State<SharedState>,
    Path(user_id): Path<Uuid>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ResponseBody<Vec<Score>>>, ErrorResponse> {
    let pool = &app_state.read().unwrap().db;
    let show_hidden = params
        .get("hidden")
        .unwrap_or(&"false".to_string())
        .to_lowercase()
        .eq("true");

    match score_service::find_by_user(user_id, show_hidden, pool) {
        Ok(scores) => Ok(Json(ResponseBody::new("Scores fetched", scores))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "",
    tag = "Score",
    operation_id = "score_store",
    request_body = ScoreDTO,
    responses(
        (status = StatusCode::CREATED, description = "Score created successfully", body = ScoreResponseBody),
        (status = StatusCode::BAD_REQUEST, description = "Invalid input", body = ErrorResponse)
    )
)]
pub async fn store(
    State(app_state): State<SharedState>,
    Json(new_score): Json<ScoreDTO>,
) -> Result<(StatusCode, Json<ResponseBody<Score>>), ErrorResponse> {
    let pool = &app_state.read().unwrap().db;

    match score_service::insert(new_score, pool) {
        Ok(scores) => Ok((
            StatusCode::CREATED,
            Json(ResponseBody::new("Score saved", scores)),
        )),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    put,
    path = "/{id}",
    tag = "Score",
    operation_id = "score_update",
    request_body = ScoreDTO,
    params(
        ("id", Path, description = "Unique id of a Score")
    ),
    responses(
        (status = StatusCode::OK, description = "Score updated successfully", body = ScoreResponseBody),
        (status = StatusCode::BAD_REQUEST, description = "Invalid input", body = ErrorResponse),
        (status = StatusCode::NOT_FOUND, description = "No score found by id", body = ErrorResponse)
    )
)]
pub async fn update(
    State(app_state): State<SharedState>,
    Path(id): Path<Uuid>,
    Json(updated_score): Json<ScoreDTO>,
) -> Result<Json<ResponseBody<Score>>, ErrorResponse> {
    let pool = &app_state.read().unwrap().db;

    match score_service::update(id, updated_score, pool) {
        Ok(scores) => Ok(Json(ResponseBody::new("Score updated", scores))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    delete,
    path = "/{id}",
    tag = "Score",
    operation_id = "score_destroy",
    params(
        ("id", Path, description = "Unique id(s) of a Score (comma seperated)")
    ),
    responses(
        (status = StatusCode::NO_CONTENT, description = "Score deleted successfully"),
        (status = StatusCode::NOT_FOUND, description = "No score found by id", body = ErrorResponse)
    )
)]
pub async fn destroy(
    State(app_state): State<SharedState>,
    Path(id): Path<String>,
) -> Result<StatusCode, ErrorResponse> {
    let pool = &app_state.read().unwrap().db;

    match score_service::delete(id, &pool) {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => Err(err),
    }
}

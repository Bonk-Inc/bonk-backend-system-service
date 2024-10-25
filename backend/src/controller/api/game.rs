use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use utoipa::{OpenApi, ToSchema};
use uuid::Uuid;

use crate::{
    models::{
        game::{Game, GameDTO},
        respone::ResponseBody,
    },
    service::game_service,
    SharedState,
};

#[derive(OpenApi)]
#[openapi(
    paths(index, show, store, update, destroy),
    components(schemas(Game, GameDTO, GameResponseBody, GamesResponseBody))
)]
pub struct GameApi;

#[derive(ToSchema)]
pub struct GameResponseBody {
    pub message: String,
    pub data: Game,
}

#[derive(ToSchema)]
pub struct GamesResponseBody {
    pub message: String,
    pub data: Vec<Game>,
}

#[utoipa::path(
    get,
    path = "",
    tag = "Game",
    operation_id = "game_index",
    responses(
        (status = StatusCode::OK, description = "Games fetched successfully", body = GamesResponseBody)
    )
)]
pub async fn index(
    State(app_state): State<SharedState>,
) -> Result<Json<ResponseBody<Vec<Game>>>, (StatusCode, String)> {
    let pool = &app_state.read().unwrap().db;

    match game_service::find_all(pool) {
        Ok(games) => Ok(Json(ResponseBody::new("Games fetched", games))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/{id}",
    tag = "Game",
    operation_id = "game_show",
    params(
        ("id", Path, description = "Unique id of a Game")
    ),
    responses(
        (status = StatusCode::OK, description = "Gam fetched successfully", body = GameResponseBody),
        (status = StatusCode::NOT_FOUND, description = "No game found by id")
    )
)]
pub async fn show(
    State(app_state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ResponseBody<Game>>, (StatusCode, String)> {
    let pool = &app_state.read().unwrap().db;

    match game_service::find_by_id(id, pool) {
        Ok(game) => Ok(Json(ResponseBody::new("Game fetched", game))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "",
    tag = "Game",
    operation_id = "game_store",
    request_body = GameDTO,
    responses(
        (status = StatusCode::CREATED, description = "Game created successfully", body = GameResponseBody),
        (status = StatusCode::BAD_REQUEST, description = "Invalid input")
    )
)]
pub async fn store(
    State(app_state): State<SharedState>,
    Json(new_game): Json<GameDTO>,
) -> Result<(StatusCode, Json<ResponseBody<Game>>), (StatusCode, String)> {
    let pool = &app_state.read().unwrap().db;

    match game_service::insert(new_game, pool) {
        Ok(game) => Ok((
            StatusCode::CREATED,
            Json(ResponseBody::new("Game created", game)),
        )),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    put,
    path = "/{id}",
    tag = "Game",
    operation_id = "game_update",
    request_body = GameDTO,
    params(
        ("id", Path, description = "Unique id of a Game")
    ),
    responses(
        (status = StatusCode::OK, description = "Game updated successfully", body = GameResponseBody),
        (status = StatusCode::BAD_REQUEST, description = "Invalid input"),
        (status = StatusCode::NOT_FOUND, description = "No game found by id")
    )
)]
pub async fn update(
    State(app_state): State<SharedState>,
    Path(id): Path<Uuid>,
    Json(updated_game): Json<GameDTO>,
) -> Result<Json<ResponseBody<Game>>, (StatusCode, String)> {
    let pool = &app_state.read().unwrap().db;

    match game_service::update(id, updated_game, pool) {
        Ok(game) => Ok(Json(ResponseBody::new("Game updated", game))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    delete,
    path = "/{id}",
    tag = "Game",
    operation_id = "game_destroy",
    params(
        ("id", Path, description = "Unique id of a Game")
    ),
    responses(
        (status = StatusCode::NO_CONTENT, description = "Game deleted successfully"),
        (status = StatusCode::NOT_FOUND, description = "No score found by id")
    )
)]
pub async fn destroy(
    State(app_state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let pool = &app_state.read().unwrap().db;

    match game_service::delete(id, pool) {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => Err(err),
    }
}

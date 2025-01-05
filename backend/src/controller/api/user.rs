use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use utoipa::{OpenApi, ToSchema};
use uuid::Uuid;

use crate::{
    error::ErrorResponse,
    models::{
        respone::ResponseBody,
        user::{User, UserDTO},
    },
    service::user_service,
    SharedState,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        index,
        store,
        update,
        destroy
    ),
    components(schemas(User, UserDTO, UserResponseBody, UsersResponseBody))
)]
pub struct UserApi;

#[derive(ToSchema)]
pub struct UsersResponseBody {
    pub message: String,
    pub data: User,
}

#[derive(ToSchema)]
pub struct UserResponseBody {
    pub message: String,
    pub data: Vec<User>,
}

#[utoipa::path(
    get,
    path = "game/{id}",
    tag = "User",
    operation_id = "user_index",
    params(
        ("id", Path, description = "Unique id of a game"),
    ),
    responses(
        (status = StatusCode::OK, description = "User fetched successfully", body = UsersResponseBody)
    )
)]
pub async fn index(
    State(app_state): State<SharedState>,
    Path(game_id): Path<Uuid>,
) -> Result<Json<ResponseBody<Vec<User>>>, ErrorResponse> {
    let pool = &app_state.read().unwrap().db;

    match user_service::find_by_game(game_id, pool) {
        Ok(users) => Ok(Json(ResponseBody::new("Users fetched", users))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "",
    tag = "User",
    operation_id = "user_store",
    request_body = UserDTO,
    responses(
        (status = StatusCode::CREATED, description = "New user created", body = UsersResponseBody),
        (status = StatusCode::BAD_REQUEST, description = "Invalid input", body = ErrorResponse)
    )
)]
pub async fn store(
    State(app_state): State<SharedState>,
    Json(new_user): Json<UserDTO>,
) -> Result<(StatusCode, Json<ResponseBody<User>>), ErrorResponse> {
    let pool = &app_state.read().unwrap().db;

    match user_service::insert(new_user, pool) {
        Ok(added_user) => Ok((
            StatusCode::CREATED,
            Json(ResponseBody::new("User created", added_user)),
        )),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    put,
    path = "/{id}",
    tag = "User",
    operation_id = "user_update",
    request_body = UserDTO,
    params(
        ("id", Path, description = "Unique id of a user"),
    ),
    responses(
        (status = StatusCode::OK, description = "User updated successfully", body = UserResponseBody),
        (status = StatusCode::BAD_REQUEST, description = "Invalid input", body = ErrorResponse),
        (status = StatusCode::NOT_FOUND, description = "No user found by id", body = ErrorResponse)
    )
)]
pub async fn update(
    State(app_state): State<SharedState>,
    Path(id): Path<Uuid>,
    Json(updated_user): Json<UserDTO>,
) -> Result<Json<ResponseBody<User>>, ErrorResponse> {
    let pool = &app_state.read().unwrap().db;

    match user_service::update(id, updated_user, pool) {
        Ok(level) => Ok(Json(ResponseBody::new("User updated", level))),
        Err(error) => Err(error),
    }
}

#[utoipa::path(
    delete,
    path = "/{id}",
    tag = "User",
    operation_id = "user_destroy",
    params(
        ("id", Path, description = "Unique id of a user"),
    ),
    responses(
        (status = StatusCode::NO_CONTENT, description = "User deleted successfully"),
        (status = StatusCode::NOT_FOUND, description = "No user found by id", body = ErrorResponse)
    )
)]
pub async fn destroy(
    State(app_state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ErrorResponse> {
    let pool = &app_state.read().unwrap().db;

    match user_service::delete(id, pool) {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => Err(err),
    }
}

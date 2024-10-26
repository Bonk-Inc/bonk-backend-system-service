use axum::{extract::{Path, State}, Json};
use utoipa::{OpenApi, ToSchema};
use uuid::Uuid;

use crate::{error::ErrorResponse, models::{respone::ResponseBody, user::{User, UserDTO}}, service::user_service, SharedState};

#[derive(OpenApi)]
#[openapi(
    paths(
        // index,
        // game_levels,
        // store,
        // update,
        // destroy
    ),
    components(schemas(User, UserDTO, UserResponseBody, UsersResponseBody))
)]
pub struct LevelApi;

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

pub async fn index(
    State(app_state): State<SharedState>,
    Path(game_id): Path<Uuid>
) -> Result<Json<ResponseBody<Vec<User>>>, ErrorResponse> {
    let pool = &app_state.read().unwrap().db;

    match user_service::find_by_game(game_id, pool) {
        Ok(users) => Ok(Json(ResponseBody::new("Users fetched", users))),
        Err(err) => Err(err),
    }
}
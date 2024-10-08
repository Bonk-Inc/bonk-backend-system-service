use actix_web::{delete, get, post, put, web, HttpResponse};
use utoipa::{OpenApi, ToSchema};
use uuid::Uuid;

use crate::{
    config::db::Pool,
    error::ServiceError,
    models::{game::{Game, GameDTO}, respone::ResponseBody},
    service::game_service,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        index,
        show,
        store,
        update,
        destroy
    ),
    components(schemas(Game, GameDTO, GameResponseBody, GamesResponseBody))
)]
pub(super) struct GameApi;

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
    path = "/game",
    tag = "Game",
    operation_id = "game_index",
    responses(
        (status = StatusCode::OK, description = "Games fetched successfully", body = GamesResponseBody)
    )
)]
#[get("/")]
pub async fn index(pool: web::Data<Pool>) -> actix_web::Result<HttpResponse, ServiceError> {
    match game_service::find_all(&pool) {
        Ok(games) => Ok(HttpResponse::Ok().json(ResponseBody::new("Games fetched", games))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/game/{id}",
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
#[get("/{id}/")]
pub async fn show(
    pool: web::Data<Pool>,
    path: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse, ServiceError> {
    match game_service::find_by_id(path.into_inner(), &pool) {
        Ok(game) => Ok(HttpResponse::Ok().json(ResponseBody::new("Game fetched", game))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/game",
    tag = "Game",
    operation_id = "game_store",
    request_body = GameDTO,
    responses(
        (status = StatusCode::CREATED, description = "Game created successfully", body = GameResponseBody),
        (status = StatusCode::BAD_REQUEST, description = "Invalid input")
    )
)]
#[post("/")]
pub async fn store(
    pool: web::Data<Pool>,
    data: web::Json<GameDTO>,
) -> actix_web::Result<HttpResponse, ServiceError> {
    match game_service::insert(data.into_inner(), &pool) {
        Ok(game) => Ok(HttpResponse::Created().json(ResponseBody::new("Game created", game))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    put,
    path = "/game/{id}",
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
#[put("/{id}/")]
pub async fn update(
    pool: web::Data<Pool>,
    data: web::Json<GameDTO>,
    path: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse, ServiceError> {
    match game_service::update(path.into_inner(), data.into_inner(), &pool) {
        Ok(game) => Ok(HttpResponse::Ok().json(ResponseBody::new("Game updated", game))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    delete,
    path = "/game/{id}",
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
#[delete("/{id}/")]
pub async fn destroy(
    pool: web::Data<Pool>,
    path: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse, ServiceError> {
    match game_service::delete(path.into_inner(), &pool) {
        Ok(_) => Ok(HttpResponse::NoContent().body("")),
        Err(err) => Err(err),
    }
}

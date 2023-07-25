use actix_web::{error, get, post, put, delete, web, HttpResponse, Responder};
use diesel::{AsChangeset, Insertable};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::{
    repository::{game_repository::GameRepository, Repository},
    schema::game,
    DbPool, models::Game,
};

#[derive(Deserialize, Insertable, AsChangeset, Validate)]
#[diesel(table_name = game)]
pub struct GameForm {
    #[validate(length(min = 5, max = 49))]
    pub name: String,
}

#[get("/")]
pub async fn index(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let games = web::block(move || {
        let mut conn = pool.get().expect("Couldn't get connection from pool");
        let repository: GameRepository = Repository::new();

        repository.find_all(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "results": games.len(),
        "data": games
    })))
}

#[get("/{id}/")]
pub async fn show(
    pool: web::Data<DbPool>,
    path: web::Path<(Uuid,)>,
) -> actix_web::Result<impl Responder> {
    let (game_id,) = path.into_inner();
    let game = web::block(move || {
        let mut conn = pool.get().expect("Couldn't get connection from pool");
        let repository: GameRepository = Repository::new();

        repository.find(&mut conn, game_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    if game.is_none() {
        return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "status": "failed",
            "message": format!("Could not find game with id '{}'", game_id.to_string())
        })));
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": { "game": game.unwrap() }
    })))
}

#[post("/")]
pub async fn store(
    pool: web::Data<DbPool>,
    data: web::Json<GameForm>,
) -> actix_web::Result<impl Responder> {

    if let Err(errors) = data.validate() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "status": "failed",
            "message": "Input data invalid",
            "errors": errors.field_errors()
        })));
    }

    let new_game = web::block(move || {
        let mut conn = pool.get().expect("Couldn't get connection from pool");
        let respository: GameRepository = Repository::new();

        respository.store(&mut conn, data)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;


    Ok(HttpResponse::Created().json(serde_json::json!({
        "status": "success",
        "data": { "game": new_game }
    })))
}

#[put("/{id}/")]
pub async fn update(
    pool: web::Data<DbPool>,
    data: web::Json<GameForm>,
    path: web::Path<(Uuid,)>
) -> actix_web::Result<impl Responder> {

    let (game_id,) = path.into_inner();
    if let Err(errors) = data.validate() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "status": "failed",
            "message": "Input data invalid",
            "errors": errors.field_errors()
        })));
    }

    let updated_game: Option<Game> = web::block(move || {
        let mut conn = pool.get().expect("Couldn't get connection from pool");
        let respository: GameRepository = Repository::new();

        respository.update(&mut conn, game_id, data)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    if updated_game.is_none() {
        return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "status": "failed",
            "message": format!("Could not update game with id '{}'", game_id.to_string())
        })));
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": { "game": updated_game.unwrap() }
    })))
}

#[delete("/{id}/")]
pub async fn destroy(
    pool: web::Data<DbPool>,
    path: web::Path<(Uuid,)>
) -> actix_web::Result<impl Responder> {
    let (game_id,) = path.into_inner();
    let is_deleted = web::block(move || {
        let mut conn = pool.get().expect("Couldn't get connection from pool");
        let repository: GameRepository = Repository::new();

        repository.drop(&mut conn, game_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    if !is_deleted {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "status": "failed",
            "message": format!("Could not delete game with id '{}'", game_id.to_string())
        })));
    }

    Ok(HttpResponse::NoContent().body(""))
}
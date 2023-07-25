use actix_web::{error, web, HttpResponse, Responder, get, post, put, delete};
use diesel::{Insertable, AsChangeset};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::{
    repository::{score_repository::ScoreRepository, Repository},
    schema::score,
    DbPool, 
};

#[derive(Deserialize, Insertable, AsChangeset, Validate)]
#[diesel(table_name = score)]
pub struct ScoreForm {
    #[validate(length(min = 5, max = 49))]
    pub username: String,
    #[serde(rename = "score")]
    pub highscore: i32,
    pub is_hidden: Option<bool>,
    pub game_id: Uuid
}

#[get("/")]
pub async fn index(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let scores = web::block(move || {
        let mut conn = pool.get().expect("Couldn't get connection from pool");
        let repository: ScoreRepository = Repository::new();

        repository.find_all(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "results": scores.len(),
        "data": scores
    })))
}

#[get("/{id}/")]
pub async fn show(
    pool: web::Data<DbPool>,
    id: web::Path<(Uuid,)>
) -> actix_web::Result<impl Responder> {
    let (score_id,) = id.into_inner();
    let score = web::block(move || {
        let mut conn = pool.get().expect("Couldn't get connection from pool");
        let repository: ScoreRepository = Repository::new();

        repository.find(&mut conn, score_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    if score.is_none() {
        return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "status": "failed",
            "message": format!("Could not find score with id '{}'", score_id.to_string())
        })));
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": { "score": score.unwrap() }
    })))
}

#[post("/")]
pub async fn store(
    pool: web::Data<DbPool>,
    data: web::Json<ScoreForm>,
) -> actix_web::Result<impl Responder> {

    if let Err(errors) = data.validate() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "status": "failed",
            "message": "Input data invalid",
            "errors": errors.field_errors()
        })));
    }

    let new_score = web::block(move || {
        let mut conn = pool.get().expect("Couldn't get connection from pool");
        let repository: ScoreRepository = Repository::new();

        repository.store(&mut conn, data)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "status": "success",
        "data": { "score": new_score }
    })))
}

#[put("/{id}/")]
pub async fn update(
    pool: web::Data<DbPool>,
    data: web::Json<ScoreForm>,
    path: web::Path<(Uuid,)>
) -> actix_web::Result<impl Responder> {

    let (score_id,) = path.into_inner();
    if let Err(errors) = data.validate() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "status": "failed",
            "message": "Input data invalid",
            "errors": errors.field_errors()
        })));
    }

    let updated_score = web::block(move || {
        let mut conn = pool.get().expect("Couldn't get connection from pool");
        let repository: ScoreRepository = Repository::new();
        
        repository.update(&mut conn, score_id, data)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    if updated_score.is_none() {
        return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "status": "failed",
            "message": format!("Could not update score with id '{}'", score_id.to_string())
        })));
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": { "score": updated_score.unwrap() }
    })))
}

#[delete("/{id}/")]
pub async fn destroy(
    pool: web::Data<DbPool>,
    path: web::Path<(Uuid,)>
) -> actix_web::Result<impl Responder> {
    let (score_id,) = path.into_inner();
    let is_deleted = web::block(move || {
        let mut conn = pool.get().expect("Couldn't get connection from pool");
        let repository: ScoreRepository = Repository::new();

        repository.drop(&mut conn, score_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    if !is_deleted {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "status": "failed",
            "message": format!("Could not delete score with id '{}'", score_id.to_string())
        })));
    }

    Ok(HttpResponse::NoContent().body(""))
}
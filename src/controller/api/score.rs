use actix_web::{error, get, post, web, HttpResponse, Responder, delete};
use diesel::{Insertable, RunQueryDsl};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::{
    models::Score,
    repository::{score_repository::ScoreRepository, Repository},
    schema::score,
    DbPool, 
};

#[derive(Deserialize, Insertable, Validate)]
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
        let conn = pool.get().expect("Couldn't get connection from pool");
        let repository: ScoreRepository = Repository::new(conn);

        repository.find_all()
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
        let conn = pool.get().expect("Couldn't get connection from pool");
        let repository: ScoreRepository = Repository::new(conn);

        repository.find(score_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(match score {
        Some(score) => HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "data": { "score": score }
        })),
        None => HttpResponse::NotFound().json(serde_json::json!({
            "status": "failed",
            "message": format!("No score found with id '{}'", score_id.to_string())
        })),
    })
}

#[post("/")]
pub async fn store(
    pool: web::Data<DbPool>,
    data: web::Json<ScoreForm>
) -> actix_web::Result<impl Responder> {
    use crate::schema::score::dsl::*;

    if let Err(errors) = data.validate() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "status": "failed",
            "message": "Input data invalid",
            "errors": errors.field_errors()
        })));
    }

    let new_score = web::block(move || {
        let mut conn = pool.get().expect("Couldn't get connection from pool");
        let result = diesel::insert_into(score)
            .values(data.0)
            .get_result::<Score>(&mut conn);

        result
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "status": "success",
        "data": { "score": new_score }
    })))
}

#[delete("/{id}/")]
pub async fn destroy(
    pool: web::Data<DbPool>,
    path: web::Path<(Uuid,)>
) -> actix_web::Result<impl Responder> {
    let (score_id,) = path.into_inner();
    let is_deleted = web::block(move || {
        let conn = pool.get().expect("Couldn't get connection from pool");
        let repository: ScoreRepository = Repository::new(conn);

        repository.drop(score_id)
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
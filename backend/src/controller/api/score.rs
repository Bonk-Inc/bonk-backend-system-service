use actix_web::{web, HttpResponse, get, post, put, delete};
use babs::respone::ResponseBody;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    config::db::Pool,
    error::ServiceError,
    models::score::ScoreDTO,
    service::score_service, 
};

#[derive(Deserialize)]
pub struct QueryParams {
    pub hidden: bool
}

#[get("/")]
pub async fn index(pool: web::Data<Pool>) -> actix_web::Result<HttpResponse, ServiceError> {
    match score_service::find_all(&pool) {
        Ok(scores) => Ok(HttpResponse::Ok().json(ResponseBody::new("Scores fetched", scores))),
        Err(err) => Err(err)
    }
}

#[get("/{id}/")]
pub async fn show(
    pool: web::Data<Pool>,
    path: web::Path<Uuid>
) -> actix_web::Result<HttpResponse, ServiceError> {
    match score_service::find_by_id(path.into_inner(), &pool) {
        Ok(score) => Ok(HttpResponse::Ok().json(ResponseBody::new("Score fetched", score))),
        Err(err) => Err(err)
    }
}

#[get("/game/{id}/")]
pub async fn game_scores(
    pool: web::Data<Pool>,
    path: web::Path<Uuid>,
    query: web::Query<QueryParams>
) -> actix_web::Result<HttpResponse, ServiceError> {
    let query_params = query.into_inner();

    match score_service::find_by_game(path.into_inner(), query_params.hidden, &pool) {
        Ok(scores) => Ok(HttpResponse::Ok().json(ResponseBody::new("Scores fetched", scores))),
        Err(err) => Err(err)
    }
}

#[get("/level/{id}/")]
pub async fn level_scores(
    pool: web::Data<Pool>,
    path: web::Path<Uuid>,
    query: web::Query<QueryParams>
) -> actix_web::Result<HttpResponse, ServiceError> {
    let query_params = query.into_inner();

    match score_service::find_by_level(path.into_inner(), query_params.hidden, &pool) {
        Ok(scores) => Ok(HttpResponse::Ok().json(ResponseBody::new("Scores fetched", scores))),
        Err(err) => Err(err)
    }
}

#[post("/")]
pub async fn store(
    pool: web::Data<Pool>,
    data: web::Json<ScoreDTO>,
) -> actix_web::Result<HttpResponse, ServiceError> {
    match score_service::insert(data.into_inner(), &pool) {
        Ok(scores) => Ok(HttpResponse::Created().json(ResponseBody::new("Score saved", scores))),
        Err(err) => Err(err)
    }
}

#[put("/{id}/")]
pub async fn update(
    pool: web::Data<Pool>,
    data: web::Json<ScoreDTO>,
    path: web::Path<Uuid>
) -> actix_web::Result<HttpResponse, ServiceError> {
    match score_service::update(path.into_inner(), data.into_inner(), &pool) {
        Ok(scores) => Ok(HttpResponse::Ok().json(ResponseBody::new("Score updated", scores))),
        Err(err) => Err(err)
    }
}

#[delete("/({id})/")]
pub async fn destroy(
    pool: web::Data<Pool>,
    path: web::Path<String>
) -> actix_web::Result<HttpResponse, ServiceError> {
    match score_service::delete(path.into_inner(), &pool) {
        Ok(_) => Ok(HttpResponse::NoContent().body("")),
        Err(err) => Err(err)
    }
}
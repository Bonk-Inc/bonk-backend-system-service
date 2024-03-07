use actix_web::{delete, get, post, put, web, HttpResponse};
use babs::respone::ResponseBody;
use uuid::Uuid;

use crate::{
    config::db::Pool,
    error::ServiceError, 
    models::level::LevelDTO, 
    service::level_service,
};

#[get("/")]
pub async fn index(pool: web::Data<Pool>) -> actix_web::Result<HttpResponse, ServiceError> {
    match level_service::find_all(&pool) {
        Ok(levels) => Ok(HttpResponse::Ok().json(ResponseBody::new("Levels fetched", levels))),
        Err(err) => Err(err),
    }
}

#[get("/game/{id}/")]
pub async fn game_levels(    
    pool: web::Data<Pool>,
    path: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse, ServiceError> {
    match level_service::find_by_game(path.into_inner(), &pool) {
        Ok(levels) => Ok(HttpResponse::Ok().json(ResponseBody::new("Levels fetched", levels))),
        Err(err) => Err(err),
    }
}

#[post("/")]
pub async fn store(
    pool: web::Data<Pool>,
    data: web::Json<LevelDTO>,
) -> actix_web::Result<HttpResponse, ServiceError> {
    match level_service::insert(data.into_inner(), &pool) {
        Ok(level) => Ok(HttpResponse::Created().json(ResponseBody::new("Level created", level))),
        Err(error) => Err(error),
    }
}

#[put("/{id}/")]
pub async fn update(
    pool: web::Data<Pool>,
    data: web::Json<LevelDTO>,
    path: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse, ServiceError> {
    match level_service::update(path.into_inner(), data.into_inner(), &pool) {
        Ok(level) => Ok(HttpResponse::Ok().json(ResponseBody::new("Level updated", level))),
        Err(error) => Err(error),
    }
}

#[delete("/{id}/")]
pub async fn delete(
    pool: web::Data<Pool>,
    path: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse, ServiceError> {
    match level_service::delete(path.into_inner(), &pool) {
        Ok(_) => Ok(HttpResponse::NoContent().body("")),
        Err(err) => Err(err),
    }
}
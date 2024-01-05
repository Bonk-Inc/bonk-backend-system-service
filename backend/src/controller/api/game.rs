use actix_web::{delete, get, post, put, web, HttpResponse};
use babs::respone::ResponseBody;
use uuid::Uuid;

use crate::{
    config::db::Pool,
    error::ServiceError,
    models::game::GameDTO,
    service::game_service,
};

#[get("/")]
pub async fn index(pool: web::Data<Pool>) -> actix_web::Result<HttpResponse, ServiceError> {
    match game_service::find_all(&pool) {
        Ok(games) => Ok(HttpResponse::Ok().json(ResponseBody::new("Games fetched", games))),
        Err(err) => Err(err),
    }
}

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

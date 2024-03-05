use actix_web::{get, post, web, HttpResponse};
use babs::respone::ResponseBody;

use crate::{config::db::Pool, error::ServiceError, models::level::LevelDTO, service::level_service};

#[get("/")]
pub async fn index(pool: web::Data<Pool>) -> actix_web::Result<HttpResponse, ServiceError> {
    match level_service::find_all(&pool) {
        Ok(scores) => Ok(HttpResponse::Ok().json(ResponseBody::new("Levels fetched", scores))),
        Err(err) => Err(err)
    }
}

#[post("/")]
pub async fn store(
    pool: web::Data<Pool>,
    data: web::Json<LevelDTO>
) -> actix_web::Result<HttpResponse, ServiceError> {
    match level_service::insert(data.into_inner(), &pool) {
        Ok(level) => Ok(HttpResponse::Created().json(ResponseBody::new("Level created", level))),
        Err(error) => Err(error),
    }
}
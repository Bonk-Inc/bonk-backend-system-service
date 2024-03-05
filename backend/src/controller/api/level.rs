use actix_web::{get, web, HttpResponse};
use babs::respone::ResponseBody;

use crate::{config::db::Pool, error::ServiceError, service::level_service};

#[get("/")]
pub async fn index(pool: web::Data<Pool>) -> actix_web::Result<HttpResponse, ServiceError> {
    match level_service::find_all(&pool) {
        Ok(scores) => Ok(HttpResponse::Ok().json(ResponseBody::new("Levels fetched", scores))),
        Err(err) => Err(err)
    }
}

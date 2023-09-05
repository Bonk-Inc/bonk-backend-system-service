use actix_web::{get, HttpResponse};

use crate::error::ServiceError;

#[get("/login")]
pub async fn login() -> Result<HttpResponse, ServiceError> {
    Ok(HttpResponse::Ok().body(""))
}
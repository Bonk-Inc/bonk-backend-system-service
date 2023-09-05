use actix_web::{get, web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::{
    config::oauth2::OAuth2Client,
    error::ServiceError,
    models::respone::ResponseBody,
    service::oauth2_service,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginQueryParams {
    code: String,
    state: String,
}

#[get("/authorize/")]
pub async fn authorize(
    oauth2_client: web::Data<OAuth2Client>,
) -> Result<HttpResponse, ServiceError> {
    let authorize_url = oauth2_service::get_authorize_url(oauth2_client);

    Ok(HttpResponse::Ok().json(ResponseBody::new("Authorize url recieved", authorize_url)))
}

#[get("/login/")]
pub async fn login(
    query: web::Query<LoginQueryParams>,
    oauth2_client: web::Data<OAuth2Client>,
) -> Result<HttpResponse, ServiceError> {
    let params = query.into_inner();
    let access_token = oauth2_service::get_access_token(params.code, oauth2_client).await;

    match access_token {
        Some(token) => Ok(HttpResponse::Ok().json(ResponseBody::new("Access Token recieved", token))),
        None => Err(ServiceError::Unautherized { 
            error_message: "Unautherized".to_string()
        })
    }
}
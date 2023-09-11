use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use log::{error, info};
use serde::{Deserialize, Serialize};

use crate::{
    models::respone::ResponseBody,
    service::oauth2_service
};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String,
    sub: String,
    exp: usize,
}

pub struct Authentication;

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware { service })
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let headers = req.headers().clone();
        let jwk_token = oauth2_service::get_jwk_tokens();
        let auth_token = headers.get("Authorization");
        if auth_token.is_none() || jwk_token.is_err() {
            info!("User authentication failed, missing token");
            let (request, _) = req.into_parts();
            let response = HttpResponse::Unauthorized()
                .json(ResponseBody::new("Invalid token", ""))
                .map_into_right_body();

            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
        }

        let token = auth_token.unwrap().to_str().unwrap().replace("Bearer ", "");
        let validation = Validation::new(Algorithm::RS256);
        let decoding_key = match jwk_token.unwrap() {
            Some(token) => match DecodingKey::from_jwk(&token) {
                Ok(key) => key,
                Err(_) => {
                    error!("Could not decode the JWK");

                    let (request, _) = req.into_parts();
                    let response: HttpResponse<EitherBody<B>> = HttpResponse::Unauthorized()
                        .json(ResponseBody::new("Error during authenticating", ""))
                        .map_into_right_body();
    
                    return Box::pin(async { Ok(ServiceResponse::new(request, response)) });    
                },
            },
            None => {
                error!("Could not get a JWK");

                let (request, _) = req.into_parts();
                let response: HttpResponse<EitherBody<B>> = HttpResponse::Unauthorized()
                    .json(ResponseBody::new("Error during authenticating", ""))
                    .map_into_right_body();

                return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
            }
        };

        match jsonwebtoken::decode::<Claims>(token.as_str(), &decoding_key, &validation) {
            Ok(_) => info!("User authenticated"),
            Err(err) => {
                info!(
                    "User authentication failed, invalid token. Reason '{:?}'",
                    err.kind()
                );
                let (request, _) = req.into_parts();
                let response = HttpResponse::Unauthorized()
                    .json(ResponseBody::new("Invalid token", ""))
                    .map_into_right_body();

                return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
            }
        }

        let res = self.service.call(req);
        Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) })
    }
}

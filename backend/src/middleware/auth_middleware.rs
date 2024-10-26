use std::env;

use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use log::{error, info};
use serde::{Deserialize, Serialize};

use crate::{error::{unauthorized_error, ErrorResponse}, service::oauth2_service};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String,
    sub: String,
    exp: usize,
}

pub async fn verify_token(
    headers: HeaderMap,
    req: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    let audience = env::var("OAUTH_CLIENT_ID").expect("OAUTH_CLIENT_ID must be set");
    let jwk_token = oauth2_service::get_jwk_tokens();
    let auth_token = headers.get("Authorization");

    if auth_token.is_none() || jwk_token.is_err() {
        return Err(unauthorized_error("Invalid token".to_string()));
    }

    let token = auth_token.unwrap().to_str().unwrap().replace("Bearer ", "");
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[audience]);

    let decoding_key = match jwk_token.unwrap() {
        Some(token) => match DecodingKey::from_jwk(&token) {
            Ok(key) => key,
            Err(_) => {
                error!("Could not decode the JWK");

                return Err(unauthorized_error("Error during authenticating".to_string()));
            }
        },
        None => {
            error!("Could not get a JWK");

            return Err(unauthorized_error("Error during authenticating".to_string()));
        }
    };

    match jsonwebtoken::decode::<Claims>(&token, &decoding_key, &validation) {
        Ok(_) => info!("User authenticated"),
        Err(err) => {
            info!(
                "User authentication failed, invalid token. Reason '{:?}'",
                err.kind()
            );

            return Err(unauthorized_error("Invalid token".to_string()));
        }
    }

    Ok(next.run(req).await)
}

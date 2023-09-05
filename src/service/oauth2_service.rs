use actix_web::web;
use oauth2::{
    basic::BasicTokenType, CsrfToken,
    reqwest::http_client, 
    AuthorizationCode,
    EmptyExtraTokenFields,
    StandardTokenResponse, RefreshToken,
};

use crate::config::oauth2::OAuth2Client;

pub fn get_authorize_url(client: web::Data<OAuth2Client>) -> String {
    let oauth2: &OAuth2Client = client.get_ref();
    let (authorize_url, _) = oauth2.client
        .authorize_url(CsrfToken::new_random)
        .url();

    authorize_url.to_string()
}

pub async fn get_access_token(
    code: String,
    client: web::Data<OAuth2Client>,
) -> Option<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>> {
    let oauth2 = client.get_ref().clone();
    let authorization_code = AuthorizationCode::new(code);

    let token_res = web::block(move || {
        oauth2.client
            .exchange_code(authorization_code)
            .request(http_client)
    })
    .await
    .expect("Could not send access token request");

    if let Ok(token) = token_res {
        return Some(token);
    }

    None
}

pub async fn refresh_token(    
    refresh: String,
    client: web::Data<OAuth2Client>,
) -> Option<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>> {
    let oauth2 = client.as_ref().clone();
    let refresh_token = RefreshToken::new(refresh);

    let token_res = web::block(move || {
        oauth2.client
            .exchange_refresh_token(&refresh_token)
            .request(http_client)
    })
    .await
    .expect("Could not send refresh access token request");

    if let Ok(token) = token_res {
        return Some(token);
    }

    None
}
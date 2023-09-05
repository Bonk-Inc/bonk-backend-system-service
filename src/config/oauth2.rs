use std::env;

use oauth2::{
    AuthUrl,
    basic::BasicClient,
    ClientId, 
    ClientSecret, 
    RedirectUrl,
    TokenUrl
};

#[derive(Clone)]
pub struct OAuth2Client {
    pub client: BasicClient,
}

impl OAuth2Client {
    pub fn new() -> OAuth2Client {
        let client_secret = env::var("OAUTH_CLIENT_SECRET").expect("OAUTH_CLIENT_SECRET must be set");
        let client_id = env::var("OAUTH_CLIENT_ID").expect("OAUTH_CLIENT_ID must be set");
        let auth_url = env::var("OAUTH_AUTH_URL").expect("OAUTH_AUTH_URL must be set");
        let token_url = env::var("OAUTH_TOKEN_URL").expect("OAUTH_TOKEN_URL must be set");
        let redirect_host = env::var("OAUTH_REDIRECT_HOST").expect("OAUTH_REDIRECT_HOST must be set");
        let redirect_url = format!("{}/{}", redirect_host, "auth/login/");

        let client = BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new(auth_url).expect("Invalid authorization endpoint URL"),
            Some(TokenUrl::new(token_url).expect("Invalid token endpoint URL")),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_url).expect("Invalid redirect URL"));

        OAuth2Client { client }
    }
}

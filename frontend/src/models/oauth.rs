use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: u32,
    pub refresh_token: String,
    pub token_type: String
}
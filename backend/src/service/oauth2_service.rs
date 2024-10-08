use std::{
    error,
    fs::File,
    io::{prelude::*, BufReader},
};

use jsonwebtoken::jwk::{Jwk, JwkSet};

use crate::JWK_FILE_PATH;

pub fn get_jwk_tokens() -> Result<Option<Jwk>, Box<dyn error::Error>> {
    let file = File::open(JWK_FILE_PATH)?;
    let mut buffer_reader = BufReader::new(file);
    let mut contents = String::new();
    buffer_reader.read_to_string(&mut contents)?;

    let tokens: JwkSet = serde_json::from_str(&contents)?;
    Ok(tokens.keys.get(0).cloned())
}

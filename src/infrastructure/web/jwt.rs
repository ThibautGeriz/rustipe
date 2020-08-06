use crate::domain::users::models::user::User;
use dotenv::dotenv;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::time::SystemTime;
use uuid::Uuid;

const NUMBER_OF_SECOND_IN_A_MONTH: usize = 60 * 60 * 24 * 30;

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
}

fn get_secrets() -> String {
    dotenv().ok();
    env::var("JWT_SECRET").expect("JWT_SECRET must be set")
}

pub fn generate_header(user: User) -> Result<String, Box<dyn Error>> {
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    let claim = Claims {
        sub: user.id.to_hyphenated().to_string(),
        exp: now.as_secs() as usize + NUMBER_OF_SECOND_IN_A_MONTH,
        iat: now.as_secs() as usize,
    };
    let token = encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(get_secrets().as_ref()),
    )?;
    Ok(token)
}

pub fn decode_header(token: &str) -> Result<Uuid, Box<dyn Error>> {
    let token = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(get_secrets().as_ref()),
        &Validation::default(),
    )?;
    Ok(Uuid::parse_str(token.claims.sub.as_str()).expect("Cannot parse UUID"))
}

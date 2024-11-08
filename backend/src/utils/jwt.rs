use axum::http::StatusCode;
use chrono::{Duration, Utc};
use dotenvy_macro::dotenv;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize,Serialize};
use sqlx::Execute;

#[derive(Deserialize, Serialize)]
pub struct Claim{
    exp:usize,
}

pub fn create_jwt() -> Result<String, StatusCode> {
    let mut now = Utc::now();
    let expire_in = Duration::seconds(30);
    now += expire_in;
    let exp = now.timestamp() as usize;
    let claim = Claim{exp: exp};
    let jwt_secret = dotenv!("JWT_SECRET");
    let key = EncodingKey::from_secret(jwt_secret.as_bytes());
    let result = encode(&Header::default(), &claim, &key);
    match result {
        Ok(token) => Ok(token),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub fn verify_jwt(token: &str) -> Result<bool,StatusCode> {
    let jwt_secret = dotenv!("JWT_SECRET");
    let key = DecodingKey::from_secret(jwt_secret.as_bytes());
    let result = decode::<Claim>(token, &key, &Validation::new(Algorithm::HS256));
    match result {
        Ok(_) => Ok(true),
        Err(error) => {
            match error.kind(){
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => Err(StatusCode::UNAUTHORIZED),
                _ => Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}
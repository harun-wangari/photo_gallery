use axum::http::StatusCode;
use chrono::{Duration, Utc};
use dotenvy_macro::dotenv;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize,Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Claim{
    exp:usize,
    pub user_id: u64,
}

pub fn create_jwt(user_id:u64) -> Result<String, StatusCode> {
    let mut now = Utc::now();
    let expire_in = Duration::hours(2);
    now += expire_in;
    let exp = now.timestamp() as usize;
    let claim = Claim{exp: exp,user_id};
    let jwt_secret = dotenv!("JWT_SECRET");
    let key = EncodingKey::from_secret(jwt_secret.as_bytes());
    let result = encode(&Header::default(), &claim, &key);
    match result {
        Ok(token) => Ok(token),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub fn verify_jwt(token: &str) -> Result<TokenData<Claim>,StatusCode> {
    let jwt_secret = dotenv!("JWT_SECRET");
    let key = DecodingKey::from_secret(jwt_secret.as_bytes());
    let result: Result<TokenData<Claim>,StatusCode> = decode::<Claim>(token, &key, &Validation::new(Algorithm::HS256))
    .map_err(|_| {StatusCode::INTERNAL_SERVER_ERROR});
    
    return result;
}
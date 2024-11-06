use axum::http::StatusCode;
use chrono::{Duration, Utc};

pub async fn create_jwt(secret:String) -> Result<String, (StatusCode,String)> {
    // let mut now = Utc::now();
    // let expire_in = Duration::seconds(30);
    // now += expire_in;
    todo!();
}
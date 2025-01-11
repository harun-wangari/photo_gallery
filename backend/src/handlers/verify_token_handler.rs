use axum::{extract::State, http::StatusCode, Extension, Json};
use sqlx::MySqlPool;

pub async fn verify_token(State(_):State<MySqlPool>,Extension(user):Extension<super::user_handler::User>) -> Json<super::user_handler::User>{
    Json(user)
}
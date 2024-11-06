use axum::{extract::State, http::StatusCode, Json};
use sqlx::{MySqlPool,FromRow};
use serde::{Serialize,Deserialize};



#[derive(Serialize, Deserialize,FromRow)]
pub struct User{
    id:i64,
    email: String,
    password: String,
    photo: String,
    token:String,
}

#[derive(Serialize, Deserialize)]
pub struct Body{
    email: String,
    password: String,
}

pub async fn user_login(State(db):State<MySqlPool>,Json(body):Json<Body>) -> Result<Json<User>, (StatusCode,String)> {
    let result = sqlx::query_as("SELECT * FROM tb_users WHERE email = ?")
    .bind(body.email)
    .fetch_one(&db)
    .await;

    match result {
        Ok(user) => Ok(Json(user)),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}
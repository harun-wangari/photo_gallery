use axum::{extract::State, Json};
use sqlx::{ FromRow, MySqlPool};
use serde::{Serialize,Deserialize};
use crate::utils::jwt::create_jwt;
use super::errors::DataError;




#[derive(Serialize,Deserialize,FromRow)]
pub struct User{
    id:Option<u64>,
    email: String,
    password: String,
    photo: String,
    token:Option<String>,
}

#[derive(Serialize,Deserialize)]
pub struct Body{
    email: String,
    password: String,
}

#[derive(Serialize,Deserialize)]
pub struct Response {
    // id: u64,
    error:String,
}



pub async fn user_login(State(db):State<MySqlPool>,Json(body):Json<Body>) -> Result<Json<User>,DataError> {
    let user = sqlx::query!("SELECT * FROM tb_users WHERE email = ?",body.email)
    .fetch_one(&db)
    .await
    .map_err(|error|
        match error {
            sqlx::Error::RowNotFound => DataError::QueryError("invalid credentials".to_string()),
            _ => DataError::QueryError("something went wrong".to_string()),
        }
    )?;

    let verify = bcrypt::verify(body.password, &user.password)?;

    if verify {
        Ok(Json(
            User{
                id:Some(user.id.clone() as u64),
                email: user.email.clone(),
                password: "".to_string(),
                photo: user.photo.clone(),
                token:Some(user.token.clone()),
            }
        ))
    }else{
         Err(DataError::QueryError("invalid credentials".to_string()))
    }
    // Ok(Json(
    //     User{
    //         id:Some(3 as u64),
    //         email: "user".to_string(),
    //         password: "".to_string(),
    //         photo: "photo".to_string(),
    //         token:Some("token".to_string()),
    //     }
    // ))
}

pub async fn create_user(State(db): State<MySqlPool>,Json(body):Json<User>) -> Result<Json<Response>,DataError>{

    let hashed_password = bcrypt::hash(body.password,14).unwrap();
    let token = create_jwt().unwrap();
    // let email = &body.email;
    sqlx::query("INSERT INTO tb_users (email,password,photo,token) VALUES (?,?,?,?)")
    .bind(&body.email)
    .bind(hashed_password)
    .bind(body.photo)
    .bind(token)
    .execute(&db)
    .await
    .map_err(|error|
        match error {
            sqlx::Error::Database(e) =>  {
                if e.code() == Some(std::borrow::Cow::Borrowed("23000"))  {
                        DataError::QueryError("Email is already registered".to_string())
                  
                }else{
                   DataError::QueryError("some thing went wrong while executing the query".to_owned()) 
                }
            },
            _ =>  DataError::QueryError("some thing when wrong".to_owned())     
        }
    )?;

    Ok(Json(
        Response{
            error:"User has been registered successfully".to_owned(),
        }
    ))  

}
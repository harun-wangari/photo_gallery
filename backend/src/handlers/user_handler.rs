use axum::{extract::State, http::StatusCode, Json};
use sqlx::{ FromRow, MySqlPool};
use serde::{Serialize,Deserialize};
use crate::utils::jwt::create_jwt;
use super::errors::DataError;




#[derive(Serialize,Deserialize,FromRow,Clone)]
pub struct User{
    pub id:Option<u64>,
    pub surname:String,
    pub lastname:String,
    pub email: String,
    pub password: String,
    pub photo: String,
    pub token:Option<String>,
}

#[derive(Serialize,Deserialize)]
pub struct Body{
    email: String,
    password: String,
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
        let token  = create_jwt(user.id.clone() as u64)
        .unwrap();
        Ok(Json(
            User{
                id:Some(user.id.clone() as u64),
                surname: user.surname.clone(),
                lastname: user.lastname.clone(),
                email: user.email.clone(),
                password: "".to_string(),
                photo: user.photo.clone(),
                token:Some(token),
            }
        ))
    }else{
         Err(DataError::QueryError("invalid credentials".to_string()))
    }
}

pub async fn create_user(State(db): State<MySqlPool>,Json(body):Json<User>) -> Result<String,DataError>{

    let hashed_password = bcrypt::hash(body.password,14).unwrap();
    // let email = &body.email;
    sqlx::query("INSERT INTO tb_files (surname,lastname,email,password,photo) VALUES (?,?,?,?,?)")
    .bind(&body.surname)
    .bind(&body.lastname)
    .bind(&body.email)
    .bind(hashed_password)
    .bind(body.photo)
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

    Ok("User has been registered successfully".to_owned(),)  

}
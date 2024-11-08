use axum::{extract::State, http::StatusCode, Json};
use sqlx::{ error::ErrorKind, Error, FromRow, MySqlPool};
use serde::{Serialize,Deserialize};
use crate::utils::jwt::{create_jwt};


#[derive(Serialize,Deserialize,FromRow)]
pub struct User{
    id:Option<i64>,
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
    id: i64,
    error:String,
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

pub async fn create_user(State(db): State<MySqlPool>,Json(body):Json<User>) -> Result<Json<Response>,Json<Response>>{
    // let user_exit = sqlx::query_as("SELECT email FROM tb_users WHERE email = ?")
    // .bind(body.email)
    // .fetch_one(&db)
    // .await;


    let hashed_password = bcrypt::hash(body.password,14).unwrap();
    let token = create_jwt().unwrap();
    // let email = &body.email;
    let result = sqlx::query("INSERT INTO tb_users (email,password,photo,token) VALUES (?,?,?,?)")
    .bind(&body.email)
    .bind(hashed_password)
    .bind(body.photo)
    .bind(token)
    .execute(&db)
    .await;

    match result {
        Ok(res) => Ok(Json(
            Response{
                id: res.last_insert_id() as i64,
                error:"User has been registered successfully".to_owned(),
            }
    )
    ),
        Err(err) => {
            match err {
                sqlx::Error::Database(e) =>  {
                    if e.code() == Some(std::borrow::Cow::Borrowed("23000"))  {
                       Err( Json(Response{
                            id: 0,
                            error:"Email has already been used".to_owned(),
                        }))
                    }else{
                        Err( Json(Response{
                            id: 0,
                            error:"some thing went wrong while executing the query".to_owned(), 
                        }))
                    }
                }
                _ =>   Err( Json(Response{
                            id: 0,
                            error:"some thing when wrong".to_owned(), // internal server error
                        }))
            }   
        }
    }
}
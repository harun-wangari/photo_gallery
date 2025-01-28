use axum::{extract::State, http::StatusCode, Json};
use sqlx::{ FromRow, MySqlPool};
use serde::{Serialize,Deserialize};
use crate::utils::jwt::create_jwt;
use super::errors::DataError;
use super::email_handler::send_email;




#[derive(Serialize,Deserialize,FromRow,Clone)]
pub struct User{
    pub id:Option<u64>,
    pub firstname:String,
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
                firstname: user.firstname.clone(),
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
    sqlx::query("INSERT INTO tb_users (firstname,lastname,email,password,photo) VALUES (?,?,?,?,?)")
    .bind(&body.firstname)
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
                   DataError::QueryError(e.to_string() )  //"some thing went wrong while executing the query".to_owned()) 
                }
            },
            _ =>  DataError::QueryError("some thing when wrong".to_owned())     
        }
    )?;

    Ok("User has been registered successfully".to_owned(),)  

}

#[derive(Serialize,Deserialize)]
pub struct ResetInfo{
    email: String,
}

pub async fn send_reset_email(State(db):State<MySqlPool>,Json(body):Json<ResetInfo>) -> Result<StatusCode,(StatusCode,String)>{
    let result = sqlx::query!("SELECT email from tb_users WHERE email = ?", body.email)
    .fetch_one(&db)
    .await;

    match result{
        Ok(record) => {
            println!("{:?}",record);
            let message : String =  "message".to_string();
            let subject : String =  "subject".to_string();
            let is_email_sent = send_email(body.email,message,subject).await;
            match is_email_sent{
                Ok(_) => Ok(StatusCode::OK),
                Err(e) => Err((StatusCode::BAD_REQUEST,e.to_string()))
            }
          
        },
        Err(e) => Err((StatusCode::BAD_REQUEST,e.to_string()))
    }
}

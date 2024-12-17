use std::{env, path::Path};

use serde::{Deserialize, Serialize};
use sqlx:: MySqlPool;
use tokio::{fs::File, io::AsyncWriteExt};
use axum::{extract::{ Multipart, State}, Json};

use super::errors::DataError;

pub async fn upload_file(State(db):State<MySqlPool>,mut multipart: Multipart) -> Result<String, DataError> {  
    while let Some(field) = multipart.next_field().await.map_err(|error|  
        match error{ 
            _ => DataError::QueryError("error while trying to upload file(s)".to_string()),
        })? {
            let field_name = field.file_name().unwrap().to_string();
            let data = field.bytes().await.unwrap();

            let album = "my_pictures".to_owned();
            let file_type = "photo".to_owned();
            let user_id = 1;  // replace with user_id
            sqlx::query!("insert into tb_files (name,user_id,album,file_type) values (?,?,?,?)",field_name,user_id,album,file_type)
            .execute(&db)
            .await
            .map_err(|error|
                match error {
                    sqlx::Error::Database(e) =>  {
                        if e.code() == Some(std::borrow::Cow::Borrowed("23000"))  {
                                DataError::QueryError(format!("image or video with filename '{}' already exists",field_name).to_string())
                          
                        }else{
                           DataError::QueryError("some thing went wrong while executing the query".to_owned()) 
                        }
                    },                   
                    _ =>  DataError::DatabaseError(error)     
                }
            )?;

            let current_path = env::current_dir().unwrap() ;
            let res = Path::new(&current_path).parent().unwrap();
            let mut file = File::create(res.join("frontend").join("public").join("images").join(field_name.clone())).await
            .map_err(|_| {
               
                DataError::QueryError(format!("Error occured while try to save '{}' file",field_name).to_string())
            })? ;
            file.write(&data).await
            .map_err(|_| {
                DataError::QueryError(format!("Error occured while try to save '{}' file",field_name).to_string())
            })?;    
        }
       

    Ok("file(s) uploaded successfully".to_owned())

}


#[derive(Deserialize, Serialize)]
pub struct User{
    id: String,
}

#[derive(Deserialize, Serialize)]
pub struct GetFilesResponse {
    id:u32,
    name:String,
    album: String,
    file_type: String,
    date_uploaded: String,
}

pub async fn get_all_files(State(db):State<MySqlPool>,Json(user):Json<User>) -> Result<Json<Vec<GetFilesResponse>>,DataError> {
    let  files = sqlx::query!("SELECT id, name, album, file_type, date_uploaded FROM tb_files WHERE user_id = ?", user.id)
    .fetch_all(&db)
    .await
    .map_err(|error| 
        match error {
            sqlx::Error::RowNotFound =>  DataError::QueryError("you have not uploaded any file".to_string()),
            _ =>  DataError::DatabaseError(error)
        }
    )?
    .into_iter()
    .map(|file| GetFilesResponse{
            id:file.id as u32,
            name:file.name,
            album: file.album,
            file_type: file.file_type,
            date_uploaded:file.date_uploaded.to_string(),
        }    
    )
    .collect();

    Ok(Json(files))
}
use std::{env, path::Path};

use serde::{Deserialize, Serialize};
use sqlx:: MySqlPool;
use tokio::{fs::File, io::AsyncWriteExt};
use axum::{extract::{ Multipart, State}, Extension, Json};

use super::errors::DataError;

#[derive(Deserialize,Serialize)]
pub struct FileUploadResponse{
    id: u32,
    name: String,
    album: String,
    file_type: String,
    date_uploaded: String,

}

#[derive(Deserialize, Serialize,Debug)]
pub struct FileInfo {
    id:u32,
    album:String,
}

#[axum::debug_handler]
pub async fn upload_file(State(db):State<MySqlPool>,Extension(user):Extension<super::user_handler::User>,mut multipart: Multipart) -> Result<Json<Vec<FileUploadResponse>>, DataError> {  
    let mut uploaded_files : Vec<FileUploadResponse> = Vec::new(); 
    let mut album = "my pics".to_owned();
    while let Some(field) = multipart.next_field().await.map_err(|error|  
        match error{ 
            _ => DataError::QueryError("error while trying to upload file(s)".to_string()),
        })? {
            let mut file_type = "photo".to_string();
            let field_name = field.name().unwrap().to_owned(); 
            if field_name.clone() != "files" {
                let field_text = field.text().await.unwrap();
                album = field_text.clone();
               
                continue;
            }
            let field_name = field.file_name().unwrap().to_string();
            let field_type = field.content_type().unwrap().to_owned();
            let data = field.bytes().await.unwrap();
            if field_type == "image//jpeg" || field_type == "image//png" || field_type == "image//gif" {
                file_type = "photo".to_owned();
            }else if  field_type == "video//mp4" || field_type == "video//mpeg" || field_type == "video//ogg" || field_type == "video//webm"{
                file_type = "video".to_owned();
            }
            let  user_id: u64 = user.id.unwrap();
            let query = sqlx::query!("insert into tb_files (name,user_id,album,file_type) values (?,?,?,?)",field_name.trim().replace(" ", "_") ,user_id,album,file_type.clone())
            .execute(&db)
            .await
            .map_err(|error|
                match error {
                    sqlx::Error::Database(e) =>  {
                        if e.to_string().contains("Duplicate entry")   {
                                DataError::QueryError(format!("image or video with filename '{}' already exists",field_name).to_string())
                          
                        }else{
                           DataError::QueryError("some thing went wrong while executing the query".to_owned()) 
                        }
                    },                   
                    _ =>  DataError::DatabaseError(error)     
                }
            )?;
            uploaded_files.push(FileUploadResponse{
                id: query.last_insert_id() as u32,
                name: field_name.clone().trim().replace(" ", "_"),
                album: album.clone(),
                file_type: file_type.clone(),
                date_uploaded: chrono::Local::now().to_string(),
            });
            let current_path = env::current_dir().unwrap() ;
            let res = Path::new(&current_path).parent().unwrap();
            let mut file = File::create(res.join("frontend").join("public").join("images").join(field_name.clone().trim().replace(" ", "_"))).await
            .map_err(|_| {
               
                DataError::QueryError(format!("Error occured while try to save '{}' file",field_name).to_string())
            })? ;
            file.write(&data).await
            .map_err(|_| {
                DataError::QueryError(format!("Error occured while try to save '{}' file",field_name).to_string())
            })?;    
        }
       

    Ok(Json(uploaded_files))

}


#[derive(Deserialize, Serialize)]
pub struct GetFilesResponse {
    id:u32,
    name:String,
    album: String,
    file_type: String,
    date_uploaded: String,
}

pub async fn get_all_files(State(db):State<MySqlPool>,Extension(user):Extension<super::user_handler::User>) -> Result<Json<Vec<GetFilesResponse>>,DataError> {
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
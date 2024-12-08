use sqlx::MySqlPool;
use tokio::{fs::File, io::AsyncWriteExt};
use axum::extract::{ Multipart, State};

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
            sqlx::query!("insert into tb_files (name,album,type) values (?,?,?)",field_name,album,file_type)
            .fetch_one(&db)
            .await
            .map_err(|error|
                match error {
                    sqlx::Error::Database(e) =>  {
                        if e.code() == Some(std::borrow::Cow::Borrowed("23000"))  {
                                DataError::QueryError(format!("image or video with file '{}' already exists",field_name).to_string())
                          
                        }else{
                           DataError::QueryError("some thing went wrong while executing the query".to_owned()) 
                        }
                    },
                    _ =>  DataError::QueryError("some thing when wrong".to_owned())     
                }
            )?;

            let mut file = File::create(format!("./images/{}",field_name)).await.unwrap();
            file.write(&data).await.unwrap();    
        }
       

    Ok("file(s) uploaded successfully".to_owned())
    
    // Err(DataError::QueryError("error".to_owned()));

}
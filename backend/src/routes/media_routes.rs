use axum::extract::State;

use super::errors::DataError;

pub async fn upload_file(State(db:State<MySqlPool>),mut multiPart: Multipart) -> Result<String, DataError> {
    Ok("uploaded".to_owned())
    
    // Err(DataError::QueryError("error".to_owned()));

}
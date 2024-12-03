use axum::{body::Body, http::{Response, StatusCode}, response::IntoResponse, Json};
use thiserror::Error;


#[derive(Error, Debug)]


pub enum DataError {
    #[error("Failed database query: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Failed query: {0}")]
    QueryError(String),

    #[error("bcrypt error: {0}")]
    Bcrypt(#[from] bcrypt::BcryptError )
    
}

impl IntoResponse for DataError {
    fn into_response(self) -> Response<Body> {
        let (status,response) = match self {
            DataError::DatabaseError(error) => server_error(error.to_string()),
            DataError::QueryError(query_error) => server_error(query_error.to_string()),
            DataError::Bcrypt(bcrypt_error) => server_error(bcrypt_error.to_string()),
        };
        (status, response).into_response()
    }
}

fn server_error(e:String) -> (StatusCode,Response<Body>){
    tracing::error!("server_error: {}", e);
    (StatusCode::UNAUTHORIZED,e.into_response())
}
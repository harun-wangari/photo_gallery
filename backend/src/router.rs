use crate::handlers::user_handler::{user_login,create_user};
use crate::handlers::media_handler::{upload_file,get_all_files};
use axum::{http::{header::CONTENT_TYPE, Method}, routing:: post, Router};
use sqlx::{MySql, Pool};
use tower_http::cors::{Any, CorsLayer};

pub fn create_router(database:Pool<MySql>) -> Router {

    let cors = CorsLayer::new()
    .allow_methods([Method::GET,Method::POST])
    .allow_origin(Any)
    .allow_headers([CONTENT_TYPE]);
    Router::new()
    .route("/api/login",post(user_login ))
    .route("/api/create_user",post(create_user))
    .route("/api/upload_files", post(upload_file))
    .route("/api/get_all_files", post(get_all_files))

    .with_state(database)
    .layer(cors)
}
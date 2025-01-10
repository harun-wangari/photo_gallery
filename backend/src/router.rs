use crate::handlers::user_handler::{user_login,create_user};
use crate::handlers::media_handler::{get_all_files, upload_file};
use crate::utils::guard::guard;
use axum::{http::{header::{CONTENT_TYPE,AUTHORIZATION}, Method}, routing:: post, Router};
use axum::middleware;
use sqlx::{MySql, Pool};
use tower_http::cors::{Any, CorsLayer};

pub fn create_router(database:Pool<MySql>) -> Router {

    let cors = CorsLayer::new()
    .allow_methods([Method::GET,Method::POST])
    .allow_origin(Any)
    .allow_headers([CONTENT_TYPE,AUTHORIZATION]);
    Router::new()
    .route("/api/upload_files", post(upload_file))
    .route("/api/get_all_files", post(get_all_files))
    .route_layer(middleware::from_fn_with_state(database.clone(),guard))
    .route("/api/login",post(user_login ))
    .route("/api/create_user",post(create_user))
    .with_state(database)
    .layer(cors)
}
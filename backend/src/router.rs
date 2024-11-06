use crate::routes::user_routes::user_login;

use axum::{http::Method, routing::{get, post}, Router};
use sqlx::{MySql, Pool};
use tower_http::cors::{Any, CorsLayer};

pub fn create_router(database:Pool<MySql>) -> Router {

    let cors = CorsLayer::new()
    .allow_methods([Method::GET,Method::POST])
    .allow_origin(Any);
    Router::new().route("/api/login",post(user_login))
    .with_state(database)
    .layer(cors)
}
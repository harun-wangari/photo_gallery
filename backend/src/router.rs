use crate::routes::user_routes::user_login;

use axum::{routing::get, Router};

pub fn create_router() -> Router {

    // let cors = CorsLayer::new()
    // .allow_methods([Method::GET,Method::POST])
    // .allow_origin(Any);
    Router::new().route("/api/login",get(user_login))
    // .layer(cors)
}
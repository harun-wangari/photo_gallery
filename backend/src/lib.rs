mod router;
mod routes;
mod database;
mod utils;

use router::create_router;
use database::database_connection;
use tokio::net::TcpListener;

pub async fn run () {
    // database connection 
    let db = database_connection().await.expect("database connection failed");

    // compose routes
    let app = create_router(db);
    let host_address = "127.0.0.1:3000".to_string();
    // create tcp listener
     let listener = TcpListener::bind(host_address).await.unwrap();

     //serve the app    
     axum::serve(listener,app)
     .await
     .unwrap();

}
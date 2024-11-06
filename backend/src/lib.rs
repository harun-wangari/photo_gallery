mod router;
mod routes;

use router::create_router;
use tokio::net::TcpListener;

pub async fn run () {
    let app = create_router();
    let host_address = "127.0.0.1:3000".to_string();
    // create tcp listener
     let listener = TcpListener::bind(host_address).await.unwrap();

     //serve the app    
     axum::serve(listener,app)
     .await
     .unwrap();

}
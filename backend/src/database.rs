use dotenvy_macro::dotenv;
use sqlx::{Error, MySqlPool};


pub async fn database_connection() -> Result<MySqlPool,Error> {
    let database_url: &str = dotenv!("DATABASE_URL");
    MySqlPool::connect(database_url).await
}
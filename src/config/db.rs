use sea_orm::{Database, DatabaseConnection, DbErr};
use dotenvy::dotenv;
use std::env;

pub async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
    dotenv().ok(); // Load .env file
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    Database::connect(database_url).await
}
use std::env;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn establish_connection() -> DatabaseConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let opt = ConnectOptions::new(database_url);
    Database::connect(opt).await.unwrap()
}
use sqlx::{Pool, Postgres};
use std::env;
use dotenv::dotenv;
use super::log::{log, LogType};

pub async fn connect_db() -> Pool<Postgres> {
    log(LogType::SETUP, "Connecting to PostgreSQL database...");
    dotenv().expect("Failed to load environment variables");
    let db_url = env::var("DATABASE_URL").expect("DATABASE URL must be set in the .env!");
    Pool::<Postgres>::connect(&db_url)
        .await
        .expect("Failed to connect to the database")
}
use dotenv;
use rocket::serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePool, types::chrono};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub address: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

pub async fn fetch_all_users() -> Result<Vec<User>, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&database_url).await?;
    let users = sqlx::query_as!(
        User,
        "select id, name, email, address, created_at from users"
    )
    .fetch_all(&pool)
    .await?;

    Ok(users)
}

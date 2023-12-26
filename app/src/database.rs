use rocket::{
    serde::{Deserialize, Serialize},
    State,
};
use sqlx::{sqlite::SqlitePool, types::chrono};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub address: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

pub async fn fetch_all_users(pool: &State<SqlitePool>) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        "select id, name, email, address, created_at from users order by id asc"
    )
    .fetch_all(pool.inner())
    .await
}

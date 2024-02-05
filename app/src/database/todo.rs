use serde::Deserialize;
use sqlx::SqlitePool;

use rocket::{serde::Serialize, State};

use crate::error::CustomError;

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoTable {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub done: bool,
}

pub async fn create_or_update_todo(
    pool: &State<SqlitePool>,
    title: &str,
    description: Option<&str>,
) -> Result<TodoTable, CustomError> {
    sqlx::query_as!(
        TodoTable,
        r#"
        INSERT INTO todo (title, description)
        VALUES ($1, $2)
        RETURNING id, title, description, done
        "#,
        title,
        description
    )
    .fetch_one(pool.inner())
    .await
    .map_err(CustomError::from)
}

pub async fn select_todo(
    pool: &State<SqlitePool>,
    limit: i64,
    done: Option<bool>,
) -> Result<Vec<TodoTable>, CustomError> {
    let done = done.unwrap_or(false);

    sqlx::query_as!(
        TodoTable,
        r#"
        SELECT id, title, description, done
        FROM todo
        WHERE done = $1
        LIMIT $2
        "#,
        done,
        limit
    )
    .fetch_all(pool.inner())
    .await
    .map_err(CustomError::from)
}

use serde::Deserialize;
use sqlx::SqlitePool;

use rocket::{
    serde::{json::Json, Serialize},
    State,
};

use crate::error::CustomError;

#[derive(Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct TodoResponse {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub done: bool,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TodoListResponse {
    pub items: Vec<TodoResponse>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct CreateOrModifyTodoRequest {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoTable {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub done: bool,
}

#[post("/todo", format = "json", data = "<todo>")]
pub async fn create_todo(
    pool: &State<SqlitePool>,
    todo: Json<CreateOrModifyTodoRequest>,
) -> Result<Json<TodoResponse>, CustomError> {
    let r = sqlx::query_as!(
        TodoTable,
        r#"
        INSERT INTO todo (title, description)
        VALUES ($1, $2)
        RETURNING id, title, description, done
        "#,
        todo.title,
        todo.description
    )
    .fetch_one(pool.inner())
    .await
    .map(|r| {
        Json(TodoResponse {
            id: r.id,
            title: r.title,
            description: r.description,
            done: r.done,
        })
    })
    .map_err(CustomError::from);

    r
}

#[get("/todo?<limit>&<done>")]
pub async fn todo_list(
    pool: &State<SqlitePool>,
    limit: Option<usize>,
    done: Option<bool>,
) -> Result<Json<TodoListResponse>, CustomError> {
    format!("limit: {:?}, done: {:?}", limit, done);

    let limit = limit.unwrap_or(10) as i64;
    let done = done.unwrap_or(false);

    let r = sqlx::query_as!(
        TodoTable,
        r#"
        SELECT id, title, description, done
        FROM todo
        WHERE done = $1
        ORDER BY created_at DESC
        LIMIT $2
        "#,
        done,
        limit
    )
    .fetch_all(pool.inner())
    .await
    .map(|r| {
        Json(TodoListResponse {
            items: r
                .into_iter()
                .map(|todo| TodoResponse {
                    id: todo.id,
                    title: todo.title,
                    description: todo.description,
                    done: todo.done,
                })
                .collect(),
        })
    })
    .map_err(CustomError::from);

    r
}

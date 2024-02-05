use serde::Deserialize;
use sqlx::SqlitePool;

use rocket::{
    serde::{json::Json, Serialize},
    State,
};

use crate::{
    database::todo::{create_or_update_todo, select_todo},
    error::CustomError,
};

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
    return create_or_update_todo(pool, &todo.title, todo.description.as_deref())
        .await
        .map(|r| {
            Json(TodoResponse {
                id: r.id,
                title: r.title,
                description: r.description,
                done: r.done,
            })
        });
}

#[get("/todo?<limit>&<done>")]
pub async fn todo_list(
    pool: &State<SqlitePool>,
    limit: Option<usize>,
    done: Option<bool>,
) -> Result<Json<TodoListResponse>, CustomError> {
    format!("limit: {:?}, done: {:?}", limit, done);

    // sqlxはコンパイル時にSQL文の検証を行うため、query_as!マクロを使用する際にはSQL文を文字列リテラルとして渡す必要がある
    let limit = limit.unwrap_or(10) as i64;

    let r = select_todo(pool, limit, done).await.map(|r| {
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
    });

    r
}

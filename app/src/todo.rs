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

#[post("/todos", format = "json", data = "<todo>")]
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

#[get("/todos?<limit>&<done>")]
pub fn todo_list(limit: Option<usize>, done: Option<bool>) -> Json<TodoListResponse> {
    format!("limit: {:?}, done: {:?}", limit, done);
    let source = vec![
        TodoResponse {
            id: i64::from_be(1),
            title: "title1".to_string(),
            description: Some("description1".to_string()),
            done: true,
        },
        TodoResponse {
            id: i64::from_be(2),
            title: "title2".to_string(),
            description: Some("description2".to_string()),
            done: false,
        },
        TodoResponse {
            id: i64::from_be(3),
            title: "title3".to_string(),
            description: Some("description3".to_string()),
            done: true,
        },
        TodoResponse {
            id: i64::from_be(4),
            title: "title4".to_string(),
            description: Some("description4".to_string()),
            done: false,
        },
    ];

    let filterd = source
        .clone()
        .into_iter()
        .filter(|todo| {
            if let Some(done) = done {
                todo.done == done
            } else {
                true
            }
        })
        .take(limit.unwrap_or(10))
        .collect::<Vec<_>>();

    Json(TodoListResponse { items: filterd })
}

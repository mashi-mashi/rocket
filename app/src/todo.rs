use uuid::Uuid;

use rocket::serde::{json::Json, Serialize};

#[derive(Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct TodoResponse {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub done: bool,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TodoListResponse {
    pub items: Vec<TodoResponse>,
}

#[get("/todo?<limit>&<done>")]
pub fn todo_list(limit: Option<usize>, done: Option<bool>) -> Json<TodoListResponse> {
    format!("limit: {:?}, done: {:?}", limit, done);
    let source = vec![
        TodoResponse {
            id: Uuid::new_v4(),
            title: "title1".to_string(),
            description: Some("description1".to_string()),
            done: true,
        },
        TodoResponse {
            id: Uuid::new_v4(),
            title: "title2".to_string(),
            description: Some("description2".to_string()),
            done: false,
        },
        TodoResponse {
            id: Uuid::new_v4(),
            title: "title3".to_string(),
            description: Some("description3".to_string()),
            done: true,
        },
        TodoResponse {
            id: Uuid::new_v4(),
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

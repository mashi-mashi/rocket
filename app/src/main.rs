use database::User;
use error::CustomError;
use rocket::{fairing::AdHoc, http::Status, serde::json::Json, State};
use sqlx::SqlitePool;
use std::env;
use todo::{create_todo, todo_list};

use crate::database::fetch_all_users;

mod database;
mod error;
mod todo;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/world")]
fn world() -> &'static str {
    "hello world22222"
}

#[get("/health")]
fn health_check() -> Status {
    Status::Ok
}

#[get("/users")]
async fn users(pool: &State<SqlitePool>) -> Result<Json<Vec<User>>, CustomError> {
    fetch_all_users(pool)
        .await
        .map_err(CustomError::from)
        .map(Json)
}

#[launch]
async fn rocket() -> _ {
    dotenv::dotenv().expect("Failed to read .env file");
    rocket::build()
        .attach(AdHoc::on_ignite("Database Pool", |rocket| async {
            let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
            let pool = SqlitePool::connect(database_url.as_str())
                .await
                .expect("データベースプールの作成に失敗しました");

            rocket.manage(pool)
        }))
        .mount("/", routes![index])
        .mount(
            "/api",
            routes![world, health_check, users, todo_list, create_todo],
        )
}

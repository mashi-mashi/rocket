use database::User;
use error::CustomError;
use rocket::serde::json::Json;

use crate::database::fetch_all_users;

mod database;
mod error;

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

#[get("/hoge")]
fn hoge() -> &'static str {
    "hoge"
}

#[get("/fuga")]
fn fuga() -> &'static str {
    "fuga"
}

#[get("/users")]
async fn users() -> Result<Json<Vec<User>>, CustomError> {
    fetch_all_users().await.map_err(CustomError::from).map(Json)
}

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().expect("Failed to read .env file");
    rocket::build()
        .mount("/", routes![index])
        .mount("/api", routes![world, hoge, fuga, users])
}

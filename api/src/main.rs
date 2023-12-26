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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/api", routes![world, hoge, fuga])
}

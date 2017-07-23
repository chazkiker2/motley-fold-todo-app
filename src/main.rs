#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_cors;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::Json;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Serialize, Deserialize)]
struct Todo {
    title: String,
}

#[post("/", data = "<todo>")]
fn create_todo(todo: Json<Todo>) -> Json<Todo> {
    todo
}

#[delete("/")]
fn delete_all() -> Json<Vec<Todo>> {
    Json(vec![])
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, create_todo, delete_all])
        .attach(rocket_cors::Cors::default())
        .launch();
}

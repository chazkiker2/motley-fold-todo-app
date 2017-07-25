#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_cors;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket::State;
use rocket::http::Status;
use rocket::response::Failure;
use rocket_contrib::Json;
use std::sync::Mutex;

#[get("/")]
fn index(state: State<Mutex<Vec<Todo>>>) -> Result<Json<Vec<Todo>>, Failure> {
    state.lock()
        .map_err(|_| Failure(Status::InternalServerError))
        .map(|todos| Json(todos.clone()))
}

#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    title: String,
}

#[post("/", data = "<todo_json>")]
fn create_todo(todo_json: Json<Todo>, state: State<Mutex<Vec<Todo>>>) -> Result<Json<Todo>, Failure> {
    state.lock()
        .map_err(|_| Failure(Status::InternalServerError))
        .map(|mut todos| {
            let todo = todo_json.into_inner();
            todos.push(todo.clone());
            Json(todo)
        })
}

#[delete("/")]
fn delete_all(state: State<Mutex<Vec<Todo>>>) -> Result<Json<Vec<Todo>>, Failure> {
    state.lock()
        .map_err(|_| Failure(Status::InternalServerError))
        .map(|mut todos| {
            todos.clear();
            Json(todos.clone())
        })
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, create_todo, delete_all])
        .attach(rocket_cors::Cors::default())
        .manage(Mutex::new(Vec::<Todo>::new()))
        .launch();
}

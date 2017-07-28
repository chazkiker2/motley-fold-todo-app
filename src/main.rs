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
use std::time::{SystemTime, UNIX_EPOCH};

#[get("/")]
fn index(state: State<Mutex<Vec<Todo>>>) -> Result<Json<Vec<Todo>>, Failure> {
    state.lock()
        .map_err(|_| Failure(Status::InternalServerError))
        .map(|todos| Json(todos.clone()))
}

#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    title: String,
    completed: Option<bool>,
    url: Option<String>,
}

#[post("/", data = "<todo_json>")]
fn create_todo(todo_json: Json<Todo>, state: State<Mutex<Vec<Todo>>>) -> Result<Json<Todo>, Failure> {
    let url = new_todo_url()?;
    state.lock()
        .map_err(|_| Failure(Status::InternalServerError))
        .map(|mut todos| {
            let mut todo = todo_json.into_inner();
            todo.completed = Some(false);
            todo.url = Some(url);
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

#[get("/<todo_id>")]
fn get_todo(todo_id: String, state: State<Mutex<Vec<Todo>>>) -> Result<Json<Todo>, Failure> {
    state.lock()
        .map_err(|_| Failure(Status::InternalServerError))
        .and_then(|todos| {
            let url = Some(todo_url(&todo_id));
            todos.iter()
                .find(|todo| todo.url == url)
                .map(|todo| Json(todo.clone()))
                .ok_or(Failure(Status::InternalServerError))
        })
}

#[delete("/<todo_id>")]
fn delete_todo(todo_id: String, state: State<Mutex<Vec<Todo>>>) -> Result<(), Failure> {
    state.lock()
        .map_err(|_| Failure(Status::InternalServerError))
        .map(|mut todos| {
            let url = Some(todo_url(&todo_id));
            todos.retain(|todo| todo.url != url);
            ()
        })
}

#[derive(Deserialize, Clone)]
struct TodoUpdate {
    title: Option<String>,
    completed: Option<bool>,
}

#[patch("/<todo_id>", data = "<todo_update>")]
fn update_todo(todo_id: String, todo_update: Json<TodoUpdate>, state: State<Mutex<Vec<Todo>>>) -> Result<Json<Todo>, Failure> {
    state.lock()
        .map_err(|_| Failure(Status::InternalServerError))
        .and_then(|mut todos| {
            let url = Some(todo_url(&todo_id));
            todos.iter_mut()
                .find(|todo| todo.url == url)
                .map(|mut todo| {
                    for title in &todo_update.title {
                        todo.title = title.clone();
                    }
                    for completed in &todo_update.completed {
                        todo.completed = Some(*completed);
                    }
                    Json(todo.clone())
                })
                .ok_or(Failure(Status::InternalServerError))
        })
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, create_todo, delete_all, get_todo, delete_todo, update_todo])
        .attach(rocket_cors::Cors::default())
        .manage(Mutex::new(Vec::<Todo>::new()))
        .launch();
}

const BASE_URL: &'static str = "http://localhost:8000";

fn new_todo_url() -> Result<String, Failure> {
    // TODO use real UUIDs? Don't hardcode base URL.
    let since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| Failure(Status::InternalServerError))?;
    Ok(format!("{}/{}.{}", BASE_URL, since_epoch.as_secs(), since_epoch.subsec_nanos()))
}

fn todo_url(todo_id: &str) -> String {
    format!("{}/{}", BASE_URL, todo_id)
}

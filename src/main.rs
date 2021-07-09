#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_cors;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;

extern crate r2d2;
extern crate r2d2_diesel;
extern crate dotenv;

mod todo_list;
mod api;
mod db;

use rocket_contrib::json::Json;
use api::{Todo, TodoList};
use todo_list::{Error, TodoCreate, TodoUpdate};

fn main() {
    let rocket = rocket::ignite();
    let base_url = rocket.config().get_str("base_url").expect("required config 'base_url'").to_owned();
    rocket
        .mount("/", routes![index, create_todo, delete_all, get_todo, delete_todo, update_todo, search_todo ])
        .attach(rocket_cors::Cors::from_options(&rocket_cors::CorsOptions::default()).expect("Failed to create rocket_cors::Cors"))
        .manage(TodoList::new(base_url, db::pool::init()))
        .launch();
}

#[get("/")]
fn index(todo_list: &TodoList) -> Result<Json<Vec<Todo>>, Error> {
    todo_list.all().map(|todos| Json(todos))
}

#[post("/", data = "<todo_json>")]
fn create_todo(todo_json: Json<TodoCreate>, todo_list: &TodoList) -> Result<Json<Todo>, Error> {
    todo_list.create_todo(&todo_json.into_inner()).map(|todo| Json(todo))
}

#[get("/<todo_id>")]
fn get_todo(todo_id: i32, todo_list: &TodoList) -> Result<Json<Todo>, Error> {
    todo_list.get_todo(todo_id).map(|todo| Json(todo))
}

#[patch("/<todo_id>", data = "<todo_update>")]
fn update_todo(todo_id: i32, todo_update: Json<TodoUpdate>, todo_list: &TodoList) -> Result<Json<Todo>, Error> {
    todo_list.update_todo(todo_id, todo_update.into_inner()).map(|todo| Json(todo))
}


/// Given a search term, find all todos with titles that contain the search term (case insensitive)
///
/// # Example
///
/// Say we had two todos, one was titled "todo_001", the other titled "todo_002".
///
/// A user could navigate to the `GET "/"` endpoint like so to get all todo items:
///
/// ```bash
/// curl -X GET http://127.0.0.1:8000/
/// ```
///
/// And they'd receive the following list of todos:
///
/// ```json
/// [
///   {
///     "url": "http://127.0.0.1:8000/1",
///     "title": "todo_001",
///     "completed": false,
///     "order": 1
///   },
///   {
///     "url": "http://127.0.0.1:8000/2",
///     "title": "todo_002",
///     "completed": false,
///     "order": 2
///   }
/// ]
/// ```
///
/// Now say that the user wanted to search for a todo containing the number "2".
/// They could call `GET "/search/2"` to obtain the following list:
///
/// Curl Call:
///
/// ```bash
/// curl -X GET http://127.0.0.1:8000/search/2
/// ```
///
/// Response body:
///
/// ```json
/// [
///   {
///     "url": "http://127.0.0.1:8000/2",
///     "title": "todo_002",
///     "completed": false,
///     "order": 2
///   }
/// ]
/// ```
#[get("/search/<search>")]
fn search_todo(search: String, todo_list: &TodoList) -> Result<Json<Vec<Todo>>, Error> {
    todo_list.search_todo(&search).map(|todos| Json(todos))
}

#[delete("/<todo_id>")]
fn delete_todo(todo_id: i32, todo_list: &TodoList) -> Result<(), Error> {
    todo_list.delete_todo(todo_id)
}

#[delete("/")]
fn delete_all(todo_list: &TodoList) -> Result<Json<Vec<Todo>>, Error> {
    todo_list.clear().map(|_| Json(vec![]))
}

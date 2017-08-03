#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_cors;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

mod todo_list;
mod api;

use rocket_contrib::Json;
use api::{Todo, TodoList};
use todo_list::{Error, TodoCreate, TodoUpdate};

fn main() {
    let rocket = rocket::ignite();
    let base_url = rocket.config().get_str("base_url").expect("required config 'base_url'").to_owned();
    rocket
        .mount("/", routes![index, create_todo, delete_all, get_todo, delete_todo, update_todo])
        .attach(rocket_cors::Cors::default())
        .manage(TodoList::new(base_url, todo_list::TodoList::new()))
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
fn get_todo(todo_id: String, todo_list: &TodoList) -> Result<Json<Todo>, Error> {
    todo_list.get_todo(&todo_id).map(|todo| Json(todo))
}

#[patch("/<todo_id>", data = "<todo_update>")]
fn update_todo(todo_id: String, todo_update: Json<TodoUpdate>, todo_list: &TodoList) -> Result<Json<Todo>, Error> {
    todo_list.update_todo(&todo_id, todo_update.into_inner()).map(|todo| Json(todo))
}

#[delete("/<todo_id>")]
fn delete_todo(todo_id: String, todo_list: &TodoList) -> Result<(), Error> {
    todo_list.delete_todo(&todo_id)
}

#[delete("/")]
fn delete_all(todo_list: &TodoList) -> Result<Json<Vec<Todo>>, Error> {
    todo_list.clear().map(|_| Json(vec![]))
}

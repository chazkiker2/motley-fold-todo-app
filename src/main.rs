#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_cors;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

mod todo_list;

use rocket::{Request, State};
use rocket::request::{FromRequest, Outcome};
use rocket_contrib::Json;
use todo_list::{Error, Todo, TodoCreate, TodoList, TodoUpdate};

#[derive(Serialize)]
struct ApiTodo {
    pub url: String,
    pub title: String,
    pub completed: bool,
    pub order: Option<u32>,
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, create_todo, delete_all, get_todo, delete_todo, update_todo])
        .attach(rocket_cors::Cors::default())
        .manage(TodoList::new())
        .launch();
}

#[get("/")]
fn index(todo_list: &TodoList) -> Result<Json<Vec<ApiTodo>>, Error> {
    todo_list.all().map(|todos| Json(todos.iter().map(ApiTodo::new).collect()))
}

#[post("/", data = "<todo_json>")]
fn create_todo(todo_json: Json<TodoCreate>, todo_list: &TodoList) -> Result<Json<ApiTodo>, Error> {
    todo_list.create_todo(&todo_json.into_inner()).map(|todo| Json(ApiTodo::new(&todo)))
}

#[get("/<todo_id>")]
fn get_todo(todo_id: String, todo_list: &TodoList) -> Result<Json<ApiTodo>, Error> {
    todo_list.get_todo(&todo_id).map(|todo| Json(ApiTodo::new(&todo)))
}

#[patch("/<todo_id>", data = "<todo_update>")]
fn update_todo(todo_id: String, todo_update: Json<TodoUpdate>, todo_list: &TodoList) -> Result<Json<ApiTodo>, Error> {
    todo_list.update_todo(&todo_id, todo_update.into_inner()).map(|todo| Json(ApiTodo::new(&todo)))
}

#[delete("/<todo_id>")]
fn delete_todo(todo_id: String, todo_list: &TodoList) -> Result<(), Error> {
    todo_list.delete_todo(&todo_id)
}

#[delete("/")]
fn delete_all(todo_list: &TodoList) -> Result<Json<Vec<ApiTodo>>, Error> {
    todo_list.clear().map(|_| Json(vec![]))
}

impl ApiTodo {
    fn new(todo: &Todo) -> ApiTodo {
        ApiTodo {
            title: todo.title.clone(),
            completed: todo.completed,
            url: todo_url(&todo.id),
            order: todo.order.clone(),
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for &'r TodoList {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> Outcome<&'r TodoList, ()> {
        request.guard::<State<TodoList>>().map(|state| state.inner())
    }
}

const BASE_URL: &'static str = "http://localhost:8000";

fn todo_url(todo_id: &str) -> String {
    format!("{}/{}", BASE_URL, todo_id)
}

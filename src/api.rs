use rocket::{Request, State};
use rocket::request::{FromRequest, Outcome};
use todo_list::{self, Error, TodoCreate, TodoUpdate};

#[derive(Serialize)]
pub struct Todo {
    pub url: String,
    pub title: String,
    pub completed: bool,
    pub order: Option<u32>,
}

pub struct TodoList {
    base_url: String,
    todo_list: todo_list::TodoList,
}

impl TodoList {
    pub fn new(base_url: String, todo_list: todo_list::TodoList) -> TodoList {
        TodoList{base_url: base_url, todo_list: todo_list}
    }

    pub fn all(&self) -> Result<Vec<Todo>, Error> {
        self.todo_list.all().map(|todos| todos.iter().map(|todo| self.adapt(&todo)).collect())
    }

    pub fn create_todo(&self, request: &TodoCreate) -> Result<Todo, Error> {
        self.todo_list.create_todo(request).map(|todo| self.adapt(&todo))
    }

    pub fn get_todo(&self, todo_id: &str) -> Result<Todo, Error> {
        self.todo_list.get_todo(todo_id).map(|todo| self.adapt(&todo))
    }

    pub fn update_todo(&self, todo_id: &str, todo_update: TodoUpdate) -> Result<Todo, Error> {
        self.todo_list.update_todo(todo_id, todo_update).map(|todo| self.adapt(&todo))
    }

    pub fn delete_todo(&self, todo_id: &str) -> Result<(), Error> {
        self.todo_list.delete_todo(todo_id)
    }

    pub fn clear(&self) -> Result<(), Error> {
        self.todo_list.clear()
    }

    fn adapt(&self, todo: &todo_list::Todo) -> Todo {
        Todo {
            title: todo.title.clone(),
            completed: todo.completed,
            url: todo_url(&self.base_url, &todo.id),
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

fn todo_url(base_url: &str, todo_id: &str) -> String {
    format!("{}/{}", base_url, todo_id)
}

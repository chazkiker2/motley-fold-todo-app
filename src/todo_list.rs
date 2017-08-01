use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct TodoList {
    todos: Mutex<Vec<Todo>>,
}

#[derive(Clone)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub completed: bool,
    pub order: Option<u32>,
}

#[derive(Deserialize)]
pub struct TodoCreate {
    pub title: String,
    pub order: Option<u32>,
}

#[derive(Deserialize, Clone)]
pub struct TodoUpdate {
    pub title: Option<String>,
    pub completed: Option<bool>,
    pub order: Option<u32>,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList{todos: Mutex::new(Vec::<Todo>::new())}
    }

    pub fn all(&self) -> Result<Vec<Todo>, Error> {
        self.todos.lock()
            .map(|todos| todos.clone())
            .map_err(|_| Error{})
    }

    pub fn create_todo(&self, request: &TodoCreate) -> Result<Todo, Error> {
        let id = new_id()?;
        self.todos.lock()
            .map_err(|_| Error{})
            .map(|mut todos| {
                let todo = Todo{
                    id: id,
                    title: request.title.clone(),
                    order: request.order.clone(),
                    completed: false,
                };
                todos.push(todo.clone());
                todo
            })
    }

    pub fn get_todo(&self, todo_id: &str) -> Result<Todo, Error> {
        self.todos.lock()
            .map_err(|_| Error{})
            .and_then(|todos| {
                todos.iter()
                    .find(|todo| todo.id == todo_id)
                    .map(|todo| todo.clone())
                    .ok_or(Error{})
            })
    }

    pub fn update_todo(&self, todo_id: &str, todo_update: TodoUpdate) -> Result<Todo, Error> {
        self.todos.lock()
            .map_err(|_| Error{})
            .and_then(|mut todos| {
                todos.iter_mut()
                    .find(|todo| todo.id == todo_id)
                    .map(|mut todo| {
                        for title in &todo_update.title {
                            todo.title = title.clone();
                        }
                        for completed in &todo_update.completed {
                            todo.completed = *completed;
                        }
                        for order in &todo_update.order {
                            todo.order = Some(*order);
                        }
                        todo.clone()
                    })
                    .ok_or(Error{})
            })
    }

    pub fn delete_todo(&self, todo_id: &str) -> Result<(), Error> {
        self.todos.lock()
            .map(|mut todos| {
                todos.retain(|todo| todo.id != todo_id);
                ()
            })
            .map_err(|_| Error{})
    }

    pub fn clear(&self) -> Result<(), Error> {
        self.todos.lock()
            .map(|mut todos| {
                todos.clear();
                ()
            })
            .map_err(|_| Error{})
    }
}

#[derive(Debug)]
pub struct Error;

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        "TodoList Error!"
    }
}
impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "TodoList Error!")
    }
}

fn new_id() -> Result<String, Error> {
    // TODO use real UUIDs?
    let since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| Error{})?;
    Ok(format!("{}.{}", since_epoch.as_secs(), since_epoch.subsec_nanos()))
}

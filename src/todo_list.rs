use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct TodoList {
    todos: Mutex<Vec<Todo>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {
    pub title: String,
    #[serde(default)]
    pub completed: bool,
    pub url: Option<String>,
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

    pub fn create_todo(&self, title: &str, order: Option<u32>) -> Result<Todo, Error> {
        let url = Some(todo_url(&new_id()?));
        self.todos.lock()
            .map_err(|_| Error{})
            .map(|mut todos| {
                let todo = Todo{url: url, title: title.to_string(), completed: false, order: order};
                todos.push(todo.clone());
                todo
            })
    }

    pub fn get_todo(&self, todo_id: &str) -> Result<Todo, Error> {
        self.todos.lock()
            .map_err(|_| Error{})
            .and_then(|todos| {
                let url = Some(todo_url(todo_id));
                todos.iter()
                    .find(|todo| todo.url == url)
                    .map(|todo| todo.clone())
                    .ok_or(Error{})
            })
    }

    pub fn update_todo(&self, todo_id: &str, todo_update: TodoUpdate) -> Result<Todo, Error> {
        self.todos.lock()
            .map_err(|_| Error{})
            .and_then(|mut todos| {
                let url = Some(todo_url(&todo_id));
                todos.iter_mut()
                    .find(|todo| todo.url == url)
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
                let url = Some(todo_url(&todo_id));
                todos.retain(|todo| todo.url != url);
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

const BASE_URL: &'static str = "http://localhost:8000";

pub fn todo_url(todo_id: &str) -> String {
    format!("{}/{}", BASE_URL, todo_id)
}

fn new_id() -> Result<String, Error> {
    // TODO use real UUIDs?
    let since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| Error{})?;
    Ok(format!("{}.{}", since_epoch.as_secs(), since_epoch.subsec_nanos()))
}

use db::pool::Pool;
use rocket::request::{FromRequest, Outcome};
use rocket::{Request, State};
use todo_list::{self, Result, TodoCreate, TodoUpdate};

#[derive(Serialize)]
pub struct Todo {
    pub url: String,
    pub title: String,
    pub completed: bool,
    pub order: Option<i32>,
}

pub struct TodoList {
    base_url: String,
    todo_list: todo_list::TodoList,
}

impl TodoList {
    pub fn new(base_url: String, pool: Pool) -> TodoList {
        TodoList {
            base_url: base_url,
            todo_list: todo_list::TodoList::new(pool),
        }
    }

    pub fn all(&self) -> Result<Vec<Todo>> {
        self.adapt_list(self.todo_list.all())
    }

    pub fn create_todo(&self, request: &TodoCreate) -> Result<Todo> {
        self.adapt_single(self.todo_list.create_todo(request))
    }

    pub fn get_todo(&self, todo_id: i32) -> Result<Todo> {
        self.adapt_single(self.todo_list.get_todo(todo_id))
    }

    /// Given a search term, find all todos with titles that contain the search term (case insensitive)
    ///
    /// # Example
    ///
    /// Say we had two todos, one was titled "todo_001", the other titled "todo_002".
    ///
    /// ```ignore
    /// // this call would return both todos
    /// let contains_todo = todo_list.search_todo("todo").unwrap();
    /// // this call would simply contain the Todo titled "todo_001"
    /// let contains_001 = todo_list.search_todo("001").unwrap();
    /// ```
    pub fn search_todo(&self, search_term: &str) -> Result<Vec<Todo>> {
        self.adapt_list(self.todo_list.search_todo(search_term))
    }

    pub fn update_todo(&self, todo_id: i32, todo_update: TodoUpdate) -> Result<Todo> {
        self.adapt_single(self.todo_list.update_todo(todo_id, todo_update))
    }

    pub fn delete_todo(&self, todo_id: i32) -> Result<()> {
        self.todo_list.delete_todo(todo_id)
    }

    pub fn clear(&self) -> Result<()> {
        self.todo_list.clear()
    }

    /// Convert a `todo_list::Todo` to a `Todo`
    fn adapt(&self, todo: &todo_list::Todo) -> Todo {
        Todo {
            title: todo.title.clone(),
            completed: todo.completed,
            url: todo_url(&self.base_url, todo.id),
            order: todo.order.clone(),
        }
    }

    /// Convert a `todo_list::Result<Vec<todo_list::Todo>>` to a `todo_list::Result<Vec<api::Todo>>`
    fn adapt_list(&self, result: Result<Vec<todo_list::Todo>>) -> Result<Vec<Todo>> {
        result.map(|todos| todos.iter().map(|todo| self.adapt(&todo)).collect())
    }

    /// Convert a `todo_list::Result<todo_list::Todo>` to a `todo_list::Result<api::Todo>`
    fn adapt_single(&self, result: Result<todo_list::Todo>) -> Result<Todo> {
        result.map(|todo| self.adapt(&todo))
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for &'r TodoList {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> Outcome<&'r TodoList, ()> {
        request
            .guard::<State<TodoList>>()
            .map(|state| state.inner())
    }
}

fn todo_url(base_url: &str, todo_id: i32) -> String {
    format!("{}/{}", base_url, todo_id)
}

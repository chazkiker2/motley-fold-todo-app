use db::pool::Pool;
use db::schema::todos;
use diesel;
use diesel::prelude::*;
use r2d2;

pub struct TodoList {
    pool: Pool,
}

#[derive(Clone, Queryable)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
    #[column_name(item_order)]
    pub order: Option<i32>,
}

#[derive(Deserialize, Insertable)]
#[table_name = "todos"]
pub struct TodoCreate {
    pub title: String,
    #[column_name(item_order)]
    pub order: Option<i32>,
}

#[derive(Deserialize, Clone, AsChangeset)]
#[table_name = "todos"]
pub struct TodoUpdate {
    pub title: Option<String>,
    pub completed: Option<bool>,
    #[column_name(item_order)]
    pub order: Option<i32>,
}

/// A `Result` type-alias for `TodoList` methods.
/// This results in more succinct return types for all methods under
/// `todo_list::TodoList` and `api::TodoList` that return a `Result` variant
pub type Result<T, E = Error> = std::result::Result<T, E>;

impl TodoList {
    pub fn new(pool: Pool) -> TodoList {
        TodoList { pool }
    }

    pub fn all(&self) -> Result<Vec<Todo>> {
        use db::schema::todos::dsl::*;
        todos
            .limit(100)
            .load(&*self.pool.get()?)
            .map_err(From::from)
    }

    pub fn create_todo(&self, request: &TodoCreate) -> Result<Todo> {
        diesel::insert(request)
            .into(todos::table)
            .get_result(&*self.pool.get()?)
            .map_err(From::from)
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
    pub fn search_todo(&self, search: &str) -> Result<Vec<Todo>> {
        use db::schema::todos::dsl::*;
        todos
            .filter(title.ilike(format!("%{}%", search)))
            .get_results(&*self.pool.get()?)
            .map_err(From::from)
    }

    pub fn get_todo(&self, todo_id: i32) -> Result<Todo> {
        use db::schema::todos::dsl::*;
        todos
            .find(todo_id)
            .first(&*self.pool.get()?)
            .map_err(From::from)
    }

    pub fn update_todo(&self, todo_id: i32, todo_update: TodoUpdate) -> Result<Todo> {
        use db::schema::todos::dsl::*;
        diesel::update(todos.find(todo_id))
            .set(&todo_update)
            .get_result(&*self.pool.get()?)
            .map_err(From::from)
    }

    pub fn delete_todo(&self, todo_id: i32) -> Result<()> {
        use db::schema::todos::dsl::*;
        diesel::delete(todos.find(todo_id))
            .execute(&*self.pool.get()?)
            .map(|_| ())
            .map_err(From::from)
    }

    pub fn clear(&self) -> Result<()> {
        use db::schema::todos::dsl::*;
        diesel::delete(todos)
            .execute(&*self.pool.get()?)
            .map(|_| ())
            .map_err(From::from)
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
impl ::std::convert::From<r2d2::GetTimeout> for Error {
    fn from(_: r2d2::GetTimeout) -> Self {
        Error {}
    }
}
impl ::std::convert::From<diesel::result::Error> for Error {
    fn from(_: diesel::result::Error) -> Self {
        Error {}
    }
}

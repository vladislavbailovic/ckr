#[derive(Debug)]
pub struct Todo {
    pub line: usize,
    pub char: usize,
    pub todo: String,
    pub context: String,
}

#[derive(Debug)]
pub struct FileTodos {
    pub path: String,
    pub todos: Vec<Todo>,
}

pub struct TodoStorage {
	pub todos: Vec<FileTodos>
}
impl TodoStorage {
	pub fn new() -> Self {
		TodoStorage {
			todos: Vec::new()
		}
	}
	pub fn add(&mut self, item: FileTodos) {
		&self.todos.push(item);
	}
	pub fn get_all(&self) -> &Vec<FileTodos> {
		return &self.todos;
	}
}

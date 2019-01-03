use crate::formatter;
use crate::todo::TodoStorage;

const MAX_WIDTH: usize = 210;

pub struct ConsoleFormatter {
    pub storage: TodoStorage,
}

impl formatter::Formatter for ConsoleFormatter {
    fn print(&self) {
        for ft in self.storage.get_all().iter() {
            for todo in ft.todos.iter() {
                let head = format!("{}: {}:{}", ft.path, todo.line, todo.char,);
                let out: String = todo.todo.chars().take(MAX_WIDTH - head.len()).collect();
                println!("{} {}", head, out);
            }
        }
    }
}

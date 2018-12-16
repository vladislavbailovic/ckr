use std::fs;

use crate::todo;

pub fn get_file_todos(filepath: &str) -> Option<todo::FileTodos> {
    let contents = fs::read_to_string(filepath).unwrap();
    let raw_todos = get_todos(contents);

    if raw_todos.len() <= 0 {
        return None;
    }

    let file_todo = todo::FileTodos {
        path: filepath.to_string(),
        todos: raw_todos,
    };

    return Some(file_todo);
}

fn get_todos(content: String) -> Vec<todo::Todo> {
    let mut todos = Vec::new();
    let todo_str = "TODO";
    content.lines().enumerate().for_each(|(idx, line)| {
        if line.contains(todo_str) {
            // @TODO: clean this up, make the parser less naive
            let q = format!(r#""{}"#, todo_str);
            if line.contains(q.as_str()) {
                return;
            }

            let char_pos = line.find(todo_str).unwrap();
            let line = line
                .replace("/*", "")
                .replace("*", "")
                .replace("*/", "")
                .replace("//", "")
                .replace("@", "");
            todos.push(todo::Todo {
                line: idx,
                char: char_pos,
                todo: line.trim().to_string(),
                context: String::new(),
            });
            // @TODO: add tags parsing (hashtags) #feature
            // @TODO: add path-to-tags #feature
        }
    });
    return todos;
}

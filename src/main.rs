extern crate walkdir;

use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

mod formatter;
mod todo;

fn main() {
    let files = get_files("/home/ve/Env/wpd/projects/plugins/shipper");
	let mut storage = todo::TodoStorage::new();
	for file in files {
		if let Some(file_todo) = get_file_todos(file.to_str().unwrap()) {
			storage.add(file_todo);
		}
	}

	let fmt = formatter::get_formatter("html", storage);
	fmt.print();
}

fn get_files(path: &str) -> Vec<PathBuf> {
	let skip_dirs = vec![ ".git", "node_modules", "build", "dist"];
	let file_types = vec![ "php", "js", "scss", "css" ];
	let mut files = Vec::new();
	let entries = WalkDir::new(path).into_iter()
		.filter_map(|e| e.ok())
		.filter(|e| !e.path().is_dir())
		.filter(|e| !is_in_blacklisted_dir(e.path(), &skip_dirs))
		.filter(|e| has_whitelisted_extension(e.path(), &file_types))
	;
	for entry in entries {
		files.push(entry.path().to_path_buf());
	}

	return files;
}

fn is_in_blacklisted_dir(path: &Path, blacklist: &Vec<&str>) -> bool {
	let dirname = path.to_str().unwrap();
	for dir in blacklist.iter() {
		let skip_path = format!("/{}/", dir);
		if dirname.contains(skip_path.as_str()) {
			return true;
		}
	}
	
	return false;
}

fn has_whitelisted_extension(path: &Path, whitelist: &Vec<&str>) -> bool {
	let extension = match path.extension() {
		None => "",
		Some(a) => a.to_str().unwrap(),
	};
	for file_type in whitelist.iter() {
		if extension == *file_type {
			return true;
		}
	}
	
	return false;
}


fn get_file_todos(filepath: &str) -> Option<todo::FileTodos> {
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
        }
    });
    return todos;
}

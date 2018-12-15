extern crate walkdir;

use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() {
    let files = list_dir("/home/ve/Env/wpd/projects/plugins/shipper");
	for file in files {
		process_file(file.to_str().unwrap());
	}
}

fn list_dir(path: &str) -> Vec<PathBuf> {
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


fn process_file(filepath: &str) {
    let path = Path::new(filepath);
    let filename = path.file_name().expect("Wut");

    let contents = fs::read_to_string(filepath).unwrap();
    let todos = get_todos_in_file(path, contents);

    if todos.len() > 0 {
        println!(
            "--- {}: {} todos ---",
            filename.to_str().unwrap(),
            todos.len()
        );
        for todo in todos {
            println!("{:?}", todo);
        }
    }
}

fn get_todos_in_file(path: &Path, content: String) -> Vec<Todo> {
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
            todos.push(Todo {
                path: path.to_str().unwrap().to_string(),
                line: idx,
                char: char_pos,
                todo: line.trim().to_string(),
                context: String::new(),
            });
        }
    });
    return todos;
}

#[derive(Debug)]
struct Todo {
    path: String,
    line: usize,
    char: usize,
    todo: String,
    context: String,
}

extern crate walkdir;

use std::fs;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    list_dir("/home/ve/Env/wpd/projects/plugins/shipper");
}

fn list_dir(path: &str) {
	let skip_dirs = [ ".git", "node_modules", "build", "dist"];
	let file_types = [ "php", "js", "scss", "css" ];
	//let mut files = Vec::new();
	let entries = WalkDir::new(path).into_iter()
		.filter_map(|e| e.ok())
		.filter(|e| {
			// Filter blacklisted directories - skip the excluded ones.
			let path = e.path();
			if path.is_dir() {
				return false;
			}
			
			let dirname = path.to_str().unwrap();
			for dir in skip_dirs.iter() {
				let skip_path = format!("/{}/", dir);
				if dirname.contains(skip_path.as_str()) {
					return false;
				}
			}
			
			return true;
		})
		.filter(|e| {
			// Filter whitelisted file types - only the included file types.
			let path = e.path();
			if path.is_dir() {
				return false;
			}

			let extension = match path.extension() {
				None => "",
				Some(a) => a.to_str().unwrap(),
			};
			for file_type in file_types.iter() {
				if extension == *file_type {
					return true;
				}
			}
			
			return false;
		})
	;
	for entry in entries {
		files.push(entry.path());
	}

	return files;
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

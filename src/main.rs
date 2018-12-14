use std::fs;
use std::path::Path;

fn main() {
	list_dir("/home/ve/Env/wpd/projects/plugins/shipper");
}

fn list_dir(path: &str) {
	let dir = Path::new(path);
	if is_skip_dir(dir) {
		return;
	}

	process_dir(dir);
	let files = process_files(dir);

/*
	if files > 0 {
		println!("- {} files found in directory: {}", files, path);
	}
*/
}

fn is_skip_dir(dir: &Path) -> bool {
	let forbidden = [ "node_modules", ".git" ];
	for &check in &forbidden {
		if dir.ends_with(check) {
			return true;
		}
	}

	return false;
}

fn process_dir(dir: &Path) {
	fs::read_dir(dir)
		.unwrap()
		.filter_map(Result::ok)
		.filter(|e| e.path().is_dir())
		.for_each(|e| list_dir(e.path().to_str().expect("Invalid entry")))
	;
}

fn process_files(dir: &Path) -> u32 {
	let mut files = 0;

	fs::read_dir(dir)
		.unwrap()
		.filter_map(Result::ok)
		.filter_map(|e| {
			e.path().to_str().and_then(|f| {
				if f.ends_with("php") { Some(e) } else { None }
			})
		})
		.for_each(|f| {
			files += 1;
			process_file(f.path().to_str().expect("Invalid path"));
		})
	;

	return files;
}

fn process_file(filepath: &str) {
	let path = Path::new(filepath);
	let filename = path.file_name().expect("Wut");
	
	let contents = fs::read_to_string(filepath).unwrap();
	let todos = find_todos(contents);

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

fn find_todos(content: String) -> Vec<Todo> {
	let mut todos = Vec::new();
	content.lines()
		.for_each(|line| {
			if line.contains("TODO") {
				todos.push(Todo {
					line: line.trim().to_string(),
					context: String::new(),
				});
			}
		})
	;
	return todos;
}

#[derive(Debug)]
struct Todo {
	line: String,
	context: String,
}

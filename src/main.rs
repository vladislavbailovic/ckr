use std::fs;
use std::path::Path;

fn process_file(filepath: &str) {
	let path = Path::new(filepath);
	let file = path.file_name().expect("Wut");
	println!("{:?}", file);
}

fn list_dir(path: &str) {
	let forbidden = [ "node_modules", ".git" ];
	let dir = Path::new(path);
	let mut process_path = true;
	for &check in &forbidden {
		if dir.ends_with(check) {
			process_path = false;
		}
	}

	if !process_path {
		return
	}

	let entries = fs::read_dir(dir)
		.expect( "Unable to read dir" );

	entries
		.filter_map(Result::ok)
		.filter(|e| e.path().is_dir())
		.for_each(|e| list_dir(e.path().to_str().expect("Invalid entry")))
	;

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

	if files > 0 {
		println!("- {} files found in directory: {}", files, path);
	}
}

fn main() {
	list_dir("/home/ve/Env/wpd/projects/plugins/shipper");
}

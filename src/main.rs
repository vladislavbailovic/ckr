use std::fs;
use std::path::Path;

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

	println!("---------------- Directory: {} -----------------------", path);
	let entries = fs::read_dir(dir)
		.expect( "Unable to read dir" );

	for entry in entries {
		if let Ok(entry) = entry {
			let path = entry.path();
			if path.is_dir() {
				list_dir(path.to_str().expect("Invalid entry"));
			} else {
				println!("{:?}", path);
			}
		}
	}
}

fn main() {
	list_dir("/home/ve/Env/wpd/projects/plugins/shipper");
}

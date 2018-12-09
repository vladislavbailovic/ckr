use std::fs;
use std::path::Path;

fn main() {
	let dir = Path::new("/home/ve/Env/wpd/projects/plugins/shipper");
	if let Ok(entries) = fs::read_dir(dir) {
		for entry in entries {
			if let Ok(entry) = entry {
				let path = entry.path();
				println!("{:?} {}", path, path.is_dir());
			}
		}
	} else {
		println!( "Unable to read dir {:?}", dir );
	}
}

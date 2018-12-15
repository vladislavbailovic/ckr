extern crate walkdir;

use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn get_files(path: &str) -> Vec<PathBuf> {
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

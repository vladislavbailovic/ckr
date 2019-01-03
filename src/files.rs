extern crate walkdir;

use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const DEFAULT_EXCLUDED_PATHS: [&str; 4] = [".git", "node_modules", "build", "dist"];
const DEFAULT_FILE_TYPES: [&str; 5] = ["rs", "php", "js", "scss", "css"];

pub fn get_files(path: &str, blacklist_str: &str, whitelist_str: &str) -> Vec<PathBuf> {
    let skip_dirs = get_augmented_list(blacklist_str, DEFAULT_EXCLUDED_PATHS.to_vec());
    let file_types = get_augmented_list(whitelist_str, DEFAULT_FILE_TYPES.to_vec());
    let mut files = Vec::new();
    let entries = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.path().is_dir())
        .filter(|e| !is_in_blacklisted_dir(e.path(), &skip_dirs))
        .filter(|e| has_whitelisted_extension(e.path(), &file_types));
    for entry in entries {
        files.push(entry.path().to_path_buf());
    }

    return files;
}

fn get_comma_separated_list<'a>(from: &'a str) -> Vec<&'a str> {
    return vec![from];
}

fn get_augmented_list<'a>(comma_separated: &'a str, default_list: Vec<&'a str>) -> Vec<&'a str> {
    let mut dflt: Vec<&str> = vec![];
    let parsed = get_comma_separated_list(comma_separated);
    dflt.extend(parsed);
    dflt.extend(default_list.to_vec());
    return dflt;
}

fn is_in_blacklisted_dir(path: &Path, blacklist: &Vec<&str>) -> bool {
    let dirname = path.to_str().unwrap();
    for dir in blacklist.iter() {
        let skip_path = format!("/{}/", dir);
        if skip_path.is_empty() {
            continue;
        }
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
        if extension.is_empty() {
            continue;
        }
        if extension == *file_type {
            return true;
        }
    }

    return false;
}

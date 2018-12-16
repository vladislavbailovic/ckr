mod files;
mod formatter;
mod parser;
mod todo;

extern crate clap;
use clap::{Arg, App};

fn main() {
    let args = App::new("ckr")
        .arg(Arg::with_name("DIRECTORY")
             .help("Directory to process")
             .takes_value(true))
        .get_matches();
    let current_dir = &std::env::current_dir().unwrap()
        .into_os_string().into_string().unwrap();
    let dir = args.value_of("DIRECTORY")
        .unwrap_or(current_dir);
    // @TODO: implement CLI args parsing
    let files = files::get_files(dir);
    let mut storage = todo::TodoStorage::new();
    for file in files {
        if let Some(file_todo) = parser::get_file_todos(file.to_str().unwrap()) {
            storage.add(file_todo);
        }
    }

    let fmt = formatter::get_formatter("console", storage);
    fmt.print();
}

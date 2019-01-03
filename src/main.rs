mod files;
mod formatter;
mod parser;
mod todo;

extern crate clap;
use clap::{App, Arg};

fn main() {
    let args = App::new("")
        .arg(
            Arg::with_name("DIRECTORY")
                .help("Directory to process")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("format")
                .takes_value(true)
                .possible_values(&["console", "html"])
                .default_value("console")
                .help("Output format")
                .value_name("FORMAT"),
        )
        .arg(
            Arg::with_name("types")
                .short("t")
                .long("types")
                .takes_value(true)
                .help("Comma-separated file extensions - preceed with '+' to add to default set")
                .value_name("EXTENSIONS"),
        )
        .arg(
            Arg::with_name("exclusions")
                .short("e")
                .long("exclusions")
                .takes_value(true)
                .help("Comma-separated path exclusions - preceed with '+' to add to default set")
                .value_name("EXCLUSIONS"),
        )
        .get_matches();
    let current_dir = &std::env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    let dir = args.value_of("DIRECTORY").unwrap_or(current_dir);
    let format = args.value_of("format").unwrap();
    let types = args.value_of("types").unwrap_or("");
    let exclusions = args.value_of("exclusions").unwrap_or("");

    let files = files::get_files(dir, exclusions, types);
    let mut storage = todo::TodoStorage::new();
    for file in files {
        if let Some(file_todo) = parser::get_file_todos(file.to_str().unwrap()) {
            storage.add(file_todo);
        }
    }

    let fmt = formatter::get_formatter(format, storage);
    fmt.print();
}

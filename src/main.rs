mod files;
mod formatter;
mod parser;
mod todo;

fn main() {
    let files = files::get_files("/home/ve/Env/wpd/projects/plugins/shipper");
    let mut storage = todo::TodoStorage::new();
    for file in files {
        if let Some(file_todo) = parser::get_file_todos(file.to_str().unwrap()) {
            storage.add(file_todo);
        }
    }

    let fmt = formatter::get_formatter("console", storage);
    fmt.print();
}

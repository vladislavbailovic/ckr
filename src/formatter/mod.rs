use crate::todo::TodoStorage;

mod console;
mod html;

pub fn get_formatter(format: &str, storage: TodoStorage) -> Box<Formatter> {
    let fmt: Box<Formatter> = match format {
        "console" => Box::new(console::ConsoleFormatter { storage: storage }),
        "html" => Box::new(html::HtmlFormatter { storage: storage }),
        _ => Box::new(console::ConsoleFormatter { storage: storage }),
    };
    return fmt;
}

pub trait Formatter {
    fn print(&self);
}

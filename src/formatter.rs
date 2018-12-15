use crate::todo::TodoStorage;

pub fn get_formatter(format: &str, storage: TodoStorage) -> Box<Formatter> {
	let fmt: Box<Formatter> = match format {
		"console" => Box::new( ConsoleFormatter{ storage: storage } ),
		"html" => Box::new( HtmlFormatter{ storage: storage } ),
		_ => Box::new( ConsoleFormatter{ storage: storage } ),
	};
	return fmt;
}

pub trait Formatter {
	fn print(&self);
}

struct HtmlFormatter {
	storage: TodoStorage
}
impl Formatter for HtmlFormatter {
	fn print(&self) {
		for ft in self.storage.get_all().iter() {
			println!("HTML: {:?}", ft);
		}
	}
}

struct ConsoleFormatter {
	storage: TodoStorage
}
impl Formatter for ConsoleFormatter {
	fn print(&self) {
		for ft in self.storage.get_all().iter() {
			println!("Console {:?}", ft);
		}
	}
}

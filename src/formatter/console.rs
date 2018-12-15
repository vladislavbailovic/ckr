use crate::todo::TodoStorage;
use crate::formatter;

pub struct ConsoleFormatter {
	pub storage: TodoStorage
}
impl formatter::Formatter for ConsoleFormatter {
	fn print(&self) {
		for ft in self.storage.get_all().iter() {
			println!("Console {:?}", ft);
		}
	}
}

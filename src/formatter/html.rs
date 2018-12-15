use crate::todo::TodoStorage;
use crate::formatter;

pub struct HtmlFormatter {
	pub storage: TodoStorage
}
impl formatter::Formatter for HtmlFormatter {
	fn print(&self) {
		for ft in self.storage.get_all().iter() {
			println!("HTML: {:?}", ft);
		}
	}
}


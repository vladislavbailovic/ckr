use crate::formatter;
use crate::todo::TodoStorage;

pub struct HtmlFormatter {
    pub storage: TodoStorage,
}
impl formatter::Formatter for HtmlFormatter {
    fn print(&self) {
        // @TODO: actually implement the HTML output
        for ft in self.storage.get_all().iter() {
            println!("HTML: {:?}", ft);
        }
    }
}

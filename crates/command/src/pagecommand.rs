//

use datamodel::{DataModel, Page};

use crate::command::Command;

#[derive(Debug)]
pub struct PageCommand {
    id: String,
    name: String,
    description: String,
}
impl Command for PageCommand {
    fn execute(&self, dm: &mut DataModel) {
        let page = Page::new(self.id.clone(), self.name.clone(), self.description.clone());
        dm.insert_page(page);
    }

    fn undo(&self, dm: &mut DataModel) {
        dm.remove_page(&self.id);
    }
}
impl PageCommand {
    pub fn new(id: String, name: String, description: String) -> Self {
        PageCommand {
            id,
            name,
            description,
        }
    }
}

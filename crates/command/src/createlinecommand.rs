//

use datamodel::{DataModel, Line};

use crate::command::Command;

#[derive(Debug)]
pub struct CreateLineCommand {
    id: String,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
}
impl Command for CreateLineCommand {
    fn execute(&self, dm: &mut DataModel) {
        let line = Line::new(self.id.clone(), self.x1, self.y1, self.x2, self.y2);
        dm.insert_node(Box::new(line));
    }

    fn undo(&self, dm: &mut DataModel) {
        dm.remove_node(&self.id);
    }
}
impl CreateLineCommand {
    pub fn new(id: String, x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        CreateLineCommand { id, x1, y1, x2, y2 }
    }
}

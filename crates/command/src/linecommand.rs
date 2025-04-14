//

use datamodel::{DataModel, Line};

use crate::command::Command;

#[derive(Debug)]
pub struct LineCommand {
    id: String,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
}
impl Command for LineCommand {
    fn execute(&self, dm: &mut DataModel) {
        let line = Line::new(self.id.clone(), self.x1, self.y1, self.x2, self.y2);
        dm.insert_node(Box::new(line));
    }

    fn undo(&self, dm: &mut DataModel) {
        dm.remove_node(&self.id);
    }
}
impl LineCommand {
    pub fn new(id: String, x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        LineCommand { id, x1, y1, x2, y2 }
    }
}

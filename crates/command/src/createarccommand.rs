//

use datamodel::{Arc, DataModel};

use crate::command::Command;

#[derive(Debug)]
pub struct CreateArcCommand {
    id: String,
    x: f64,
    y: f64,
    r: f64,
    angle_start: f64,
    angle_end: f64,
}
impl Command for CreateArcCommand {
    fn execute(&self, dm: &mut DataModel) {
        let arc = Arc::new(
            self.id.clone(),
            self.x,
            self.y,
            self.r,
            self.angle_start,
            self.angle_end,
        );
        dm.insert_node(Box::new(arc));
    }

    fn undo(&self, dm: &mut DataModel) {
        dm.remove_node(&self.id);
    }
}
impl CreateArcCommand {
    pub fn new(id: String, x: f64, y: f64, r: f64, angle_start: f64, angle_end: f64) -> Self {
        CreateArcCommand {
            id,
            x,
            y,
            r,
            angle_start,
            angle_end,
        }
    }
}

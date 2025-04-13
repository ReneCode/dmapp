//

use datamodel::DataModel;

use crate::command::Command;

#[derive(Debug, Default)]
pub struct CommandHandler {
    undo_stack: Vec<Box<dyn Command>>,
}
impl CommandHandler {
    pub fn new() -> Self {
        CommandHandler {
            undo_stack: Vec::new(),
        }
    }

    pub fn execute(&mut self, dm: &mut DataModel, cmd: Box<dyn Command>) {
        cmd.execute(dm);
        self.undo_stack.push(cmd);
    }
    pub fn undo(&mut self, dm: &mut DataModel) {
        if let Some(cmd) = self.undo_stack.pop() {
            cmd.undo(dm);
        } else {
            println!("No commands to undo");
        }
    }
    pub fn list_commands(&self) {
        for cmd in &self.undo_stack {
            println!("{:?}", cmd);
        }
    }
}

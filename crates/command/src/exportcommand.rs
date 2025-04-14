//

use std::{fs::File, io::Write};

use datamodel::DataModel;

use crate::command::Command;

#[derive(Debug)]
pub struct ExportCommand {
    pub filename: String,
}

impl Command for ExportCommand {
    fn execute(&self, data_model: &mut DataModel) {
        // Implement the export logic here

        let serialized = serde_json::to_string(data_model).unwrap();

        let mut file = File::create(self.filename.as_str()).expect("Unable to create file");
        file.write_all(serialized.as_bytes())
            .expect("Unable to write data to file");
    }
}
impl ExportCommand {
    pub fn new(filename: String) -> Self {
        ExportCommand { filename }
    }
}

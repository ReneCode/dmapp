use datamodel::DataModel;

use crate::{command::Command, ArcCommand, ExportCommand, LineCommand, PageCommand};

pub struct CommandLine {}

impl CommandLine {
    pub fn parse(data_model: &mut DataModel, line: &str) -> Result<Box<dyn Command>, String> {
        let mut parts = line.split_whitespace();
        let command_name = parts.next().unwrap_or("").to_string();

        match command_name.to_lowercase().as_str() {
            "line" => {
                let x1: f64 = parts.next().unwrap_or("0").parse().unwrap_or(0.0);
                let y1: f64 = parts.next().unwrap_or("0").parse().unwrap_or(0.0);
                let x2: f64 = parts.next().unwrap_or("0").parse().unwrap_or(0.0);
                let y2: f64 = parts.next().unwrap_or("0").parse().unwrap_or(0.0);

                let cmd = LineCommand::new(data_model.next_id(), x1, y1, x2, y2);
                return Ok(Box::new(cmd));
            }
            "arc" => {
                let x: f64 = parts.next().unwrap_or("0").parse().unwrap_or(0.0);
                let y: f64 = parts.next().unwrap_or("0").parse().unwrap_or(0.0);
                let r: f64 = parts.next().unwrap_or("0").parse().unwrap_or(0.0);
                let angle_start: f64 = parts.next().unwrap_or("0").parse().unwrap_or(0.0);
                let angle_end: f64 = parts.next().unwrap_or("360").parse().unwrap_or(0.0);

                let cmd = ArcCommand::new(data_model.next_id(), x, y, r, angle_start, angle_end);
                return Ok(Box::new(cmd));
            }
            "page" => {
                let name = parts.next().unwrap_or("new page").to_string();
                let description = parts.next().unwrap_or("page description").to_string();
                let cmd = PageCommand::new(data_model.next_id(), name, description);
                return Ok(Box::new(cmd));
            }
            "export" => {
                let filename = parts.next().unwrap_or("datamodel.json").to_string();
                return Ok(Box::new(ExportCommand::new(filename)));
            }
            _ => Err("Unknown command".to_string()),
        }
        // match command_name.as_str() {
        //     "create" => excecute_create(&mut data_model, &mut command_handler, &mut parts),
        //     "list" => execute_list(&data_model, &command_handler, &mut parts),
        //     "save" => save_data(&data_model),
        //     "undo" => command_handler.undo(&mut data_model),
        //     _ => {
        //         eprintln!("Error: Unknown command: {}", command_name);
        //     }
        // }
    }
}

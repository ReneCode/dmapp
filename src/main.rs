use core::f64;
use core::fmt::Debug;
use std::{
    fs::File,
    io::{self, BufRead, Write},
    str::SplitWhitespace,
};

mod arc;
mod command;
mod datamodel;
mod line;
mod node;
mod page;

use arc::Arc;
use command::Command;
use command::CreatePageCommand;
use datamodel::DataModel;
use line::Line;
use node::{Node, NodeType};

// #[derive(Debug)]
// struct CommandHandler {
//     undo_stack: Vec<Box<dyn Command>>,
// }

// impl CommandHandler {
//     pub fn execute(&mut self, dm: &mut DataModel, command: Box<dyn Command>) {
//         command.execute(dm);
//         self.undo_stack.push(command);
//     }
//     pub fn undo(&mut self, dm: &mut DataModel) {
//         if let Some(cmd) = self.undo_stack.pop() {
//             cmd.undo(dm);
//         } else {
//             println!("No commands to undo");
//         }
//     }
//     pub fn list_commands(&self) {
//         for cmd in &self.undo_stack {
//             println!("{:?}", cmd);
//         }
//     }
// }

#[derive(Debug)]
struct CreateLineCommand {
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
    fn new(id: String, x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        CreateLineCommand { id, x1, y1, x2, y2 }
    }
}

#[derive(Debug)]
struct CreateArcCommand {
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
    fn new(id: String, x: f64, y: f64, r: f64, angle_start: f64, angle_end: f64) -> Self {
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

fn save_data(dm: &DataModel) {
    let serialized = serde_json::to_string(dm).unwrap();

    let mut file = File::create("datamodel.json").expect("Unable to create file");
    file.write_all(serialized.as_bytes())
        .expect("Unable to write data to file");
}

fn excecute_create(dm: &mut DataModel, parameter: &mut SplitWhitespace<'_>) {
    let node_type_str = parameter.next().unwrap_or("").to_string();
    let node_type = NodeType::from(node_type_str.as_str());
    match node_type {
        NodeType::Page => {
            let cmd = CreatePageCommand::new(
                dm.next_id(),
                parameter.next().unwrap_or("new-page").to_string(),
                parameter.next().unwrap_or("page-description").to_string(),
            );

            dm.execute_command(Box::new(cmd));
        }
        NodeType::Line => {
            let x1: f64 = parameter.next().unwrap_or("0.0").parse().unwrap_or(0.0);
            let y1: f64 = parameter.next().unwrap_or("0.0").parse().unwrap_or(0.0);
            let x2: f64 = parameter.next().unwrap_or("0.0").parse().unwrap_or(0.0);
            let y2: f64 = parameter.next().unwrap_or("0.0").parse().unwrap_or(0.0);

            let cmd = CreateLineCommand::new(dm.next_id(), x1, y1, x2, y2);
            dm.execute_command(Box::new(cmd));
        }

        NodeType::Arc => {
            let x: f64 = parameter.next().unwrap_or("0.0").parse().unwrap_or(0.0);
            let y: f64 = parameter.next().unwrap_or("0.0").parse().unwrap_or(0.0);
            let r: f64 = parameter.next().unwrap_or("0.0").parse().unwrap_or(0.0);
            let angle_start: f64 = parameter.next().unwrap_or("0.0").parse().unwrap_or(0.0);
            let angle_end: f64 = parameter.next().unwrap_or("360.0").parse().unwrap_or(0.0);

            let cmd = CreateArcCommand::new(dm.next_id(), x, y, r, angle_start, angle_end);
            dm.execute_command(Box::new(cmd));
        } // _ => {
          //     eprintln!("Error: Unknown node type: {}", node_type);
          // }
    }
}

fn execute_list(dm: &DataModel, parameter: &mut SplitWhitespace<'_>) {
    let node_type = parameter.next().unwrap_or("").to_string();

    match node_type.as_str() {
        "commands" => {
            dm.list_commands();
        }
        "pages" => {
            for page in dm.get_pages() {
                println!("Page ID: {}, Name: {}", page.get_id(), page.get_name());
            }
        }
        "page" => {
            let id = parameter.next().unwrap_or("").to_string();
            if let Some(page) = dm.get_page(&id) {
                println!("Page ID: {}, Name: {}", page.get_id(), page.get_name());
            } else {
                eprintln!("Error: Page with ID {} not found", id);
            }
        }
        "lines" => {
            for node in dm.get_nodes() {
                match node.get_node_type() {
                    NodeType::Line => {
                        if let Some(line) = node.as_any().downcast_ref::<Line>() {
                            println!(
                                "Line ID: {}, Coordinates: ({}, {}), ({}, {})",
                                line.get_id(),
                                line.get_x1(),
                                line.get_y1(),
                                line.get_x2(),
                                line.get_y2()
                            );
                        }
                    }
                    _ => {
                        eprintln!("Error: unhandled node type {}", node.get_node_type());
                    }
                }
            }
        }

        _ => {
            eprintln!("Error: Unknown node type: {}", node_type);
        }
    }
}

// fn undo(dm: &mut DataModel) {
//     if let Some(cmd) = dm.undo_stack.last() {
//         cmd.undo(&mut dm);
//         println!("Undoing last command: {:?}", cmd);
//     } else {
//         println!("No commands to undo");
//     }
// }

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    println!("Please enter command ('exit' or Ctrl+D to end):");

    let mut data_model = DataModel::default();

    for line in handle.lines() {
        match line {
            Ok(line) => {
                if line == "exit" {
                    break;
                }

                let mut parts = line.split_whitespace();
                let command_name = parts.next().unwrap_or("").to_string();

                match command_name.as_str() {
                    "create" => excecute_create(&mut data_model, &mut parts),
                    "list" => execute_list(&data_model, &mut parts),
                    "save" => save_data(&data_model),
                    "undo" => data_model.undo(),
                    _ => {
                        eprintln!("Error: Unknown command: {}", command_name);
                    }
                }
            }
            Err(error) => eprintln!("Error reading line: {}", error),
        }
    }
}

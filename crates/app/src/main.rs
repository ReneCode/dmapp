use core::f64;
use std::{
    fs::File,
    io::{self, BufRead, Write},
    str::SplitWhitespace,
};

use command::{CommandHandler, CreateArcCommand, CreateLineCommand, CreatePageCommand};
use datamodel::{DataModel, Line, Node, NodeType};

fn save_data(dm: &DataModel) {
    let serialized = serde_json::to_string(dm).unwrap();

    let mut file = File::create("datamodel.json").expect("Unable to create file");
    file.write_all(serialized.as_bytes())
        .expect("Unable to write data to file");
}

fn excecute_create(
    dm: &mut DataModel,
    command_handler: &mut CommandHandler,
    parameter: &mut SplitWhitespace<'_>,
) {
    let node_type_str = parameter.next().unwrap_or("").to_string();
    let node_type = NodeType::from(node_type_str.as_str());
    match node_type {
        NodeType::Page => {
            let cmd = CreatePageCommand::new(
                dm.next_id(),
                parameter.next().unwrap_or("new-page").to_string(),
                parameter.next().unwrap_or("page-description").to_string(),
            );

            command_handler.execute(dm, Box::new(cmd));
        }
        NodeType::Line => {
            let x1: f64 = parameter.next().unwrap_or("0.0").parse().unwrap_or(0.0);
            let y1: f64 = parameter.next().unwrap_or("0.0").parse().unwrap_or(0.0);
            let x2: f64 = parameter.next().unwrap_or("0.0").parse().unwrap_or(0.0);
            let y2: f64 = parameter.next().unwrap_or("0.0").parse().unwrap_or(0.0);

            let cmd = CreateLineCommand::new(dm.next_id(), x1, y1, x2, y2);
            command_handler.execute(dm, Box::new(cmd));
        }

        NodeType::Arc => {
            let x: f64 = parameter.next().unwrap_or("0.0").parse().unwrap_or(0.0);
            let y: f64 = parameter.next().unwrap_or("0.0").parse().unwrap_or(0.0);
            let r: f64 = parameter.next().unwrap_or("0.0").parse().unwrap_or(0.0);
            let angle_start: f64 = parameter.next().unwrap_or("0.0").parse().unwrap_or(0.0);
            let angle_end: f64 = parameter.next().unwrap_or("360.0").parse().unwrap_or(0.0);

            let cmd = CreateArcCommand::new(dm.next_id(), x, y, r, angle_start, angle_end);
            command_handler.execute(dm, Box::new(cmd));
        } // _ => {
          //     eprintln!("Error: Unknown node type: {}", node_type);
          // }
    }
}

fn execute_list(
    dm: &DataModel,
    command_handler: &CommandHandler,
    parameter: &mut SplitWhitespace<'_>,
) {
    let node_type = parameter.next().unwrap_or("").to_string();

    match node_type.as_str() {
        "commands" => {
            command_handler.list_commands();
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

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    println!("Please enter command ('exit' or Ctrl+D to end):");

    let mut data_model = DataModel::default();
    let mut command_handler = CommandHandler::new();

    for line in handle.lines() {
        match line {
            Ok(line) => {
                if line == "exit" {
                    break;
                }

                let mut parts = line.split_whitespace();
                let command_name = parts.next().unwrap_or("").to_string();

                match command_name.as_str() {
                    "create" => excecute_create(&mut data_model, &mut command_handler, &mut parts),
                    "list" => execute_list(&data_model, &command_handler, &mut parts),
                    "save" => save_data(&data_model),
                    "undo" => command_handler.undo(&mut data_model),
                    _ => {
                        eprintln!("Error: Unknown command: {}", command_name);
                    }
                }
            }
            Err(error) => eprintln!("Error reading line: {}", error),
        }
    }
}

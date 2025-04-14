use core::f64;
use std::{
    fs::File,
    io::{self, BufRead, Write},
    str::SplitWhitespace,
};

use command::{ArcCommand, CommandHandler, CommandLine, LineCommand, PageCommand};
use datamodel::{DataModel, Line, Node, NodeType};

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

    // let commandline = command::CommandLine::new();

    for line in handle.lines() {
        match line {
            Ok(line) => {
                if line == "quit" {
                    break;
                }

                CommandLine::parse(&mut data_model, line.as_str())
                    .map(|cmd| {
                        command_handler.execute(&mut data_model, cmd);
                    })
                    .unwrap_or_else(|err| {
                        eprintln!("Error: {}", err);
                    });

                // let mut parts = line.split_whitespace();
                // let command_name = parts.next().unwrap_or("").to_string();

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
            Err(error) => eprintln!("Error reading line: {}", error),
        }
    }
}

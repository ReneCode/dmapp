use core::f64;
use core::fmt::Debug;
use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    io::{self, BufRead, Write},
    str::SplitWhitespace,
};

use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};

mod commandparser;

#[derive(Debug, Serialize, Deserialize)]
enum NodeType {
    // Root,
    Page,
    Line,
    Arc,
}
impl Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeType::Page => write!(f, "Page"),
            NodeType::Line => write!(f, "Line"),
            NodeType::Arc => write!(f, "Arc"),
        }
    }
}
impl From<&str> for NodeType {
    fn from(s: &str) -> Self {
        match s {
            "page" => NodeType::Page,
            "line" => NodeType::Line,
            "arc" => NodeType::Arc,
            _ => panic!("Invalid node type"),
        }
    }
}

trait Node: std::fmt::Debug {
    fn get_id(&self) -> &str;
    fn get_node_type(&self) -> &NodeType;
    fn as_any(&self) -> &dyn std::any::Any;
}

#[derive(Debug, Serialize, Deserialize)]
struct Page {
    node_type: NodeType,
    id: String,
    name: String,
    description: String,
}

impl Node for Page {
    fn get_id(&self) -> &str {
        self.id.as_str()
    }

    fn get_node_type(&self) -> &NodeType {
        &self.node_type
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Line {
    node_type: NodeType,
    id: String,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
}

impl Node for Line {
    fn get_id(&self) -> &str {
        self.id.as_str()
    }

    fn get_node_type(&self) -> &NodeType {
        &self.node_type
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Arc {
    node_type: NodeType,
    id: String,
    x: f64,
    y: f64,
    r: f64,
    angle_start: f64,
    angle_end: f64,
}
impl Node for Arc {
    fn get_id(&self) -> &str {
        self.id.as_str()
    }

    fn get_node_type(&self) -> &NodeType {
        &self.node_type
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
impl Arc {
    fn new(id: String, x: f64, y: f64, r: f64, angle_start: f64, angle_end: f64) -> Self {
        Arc {
            node_type: NodeType::Arc,
            id,
            x,
            y,
            r,
            angle_start,
            angle_end,
        }
    }
}

impl Page {
    fn new(id: String, name: String, content: String) -> Self {
        Page {
            node_type: NodeType::Page,
            id,
            name,
            description: content,
        }
    }
}
impl Line {
    fn new(id: String, x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Line {
            node_type: NodeType::Line,
            id,
            x1,
            y1,
            x2,
            y2,
        }
    }
}

#[derive(Debug)]
struct IdCounter {
    counter: u64,
}
impl Default for IdCounter {
    fn default() -> Self {
        IdCounter { counter: 0 }
    }
}
impl IdCounter {
    fn next(&mut self) -> String {
        self.counter += 1;
        self.counter.to_string()
    }
}

#[derive(Debug)]
struct DataModel {
    pages: HashMap<String, Page>,
    nodes: HashMap<String, Box<dyn Node>>,
    // #[serde(skip_serializing)]
    id_counter: IdCounter,
    undo_stack: Vec<Box<dyn Command>>,
}
impl DataModel {
    fn add_command(&mut self, cmd: Box<dyn Command>) {
        self.undo_stack.push(cmd);
    }
}

impl Default for DataModel {
    fn default() -> Self {
        DataModel {
            id_counter: IdCounter::default(),
            pages: HashMap::new(),
            nodes: HashMap::new(),
            undo_stack: Vec::new(),
        }
    }
}

impl Serialize for DataModel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("DataModel", 3)?;

        state.serialize_field("id_counter", &self.id_counter.counter)?;
        // serialize only the values, the keys are not needed
        let serialized_pages: Vec<_> = self
            .pages
            .iter()
            .map(|(id, page)| {
                let page_data = serde_json::to_value(page).map_err(serde::ser::Error::custom)?;
                Ok(page_data)
            })
            .collect::<Result<_, _>>()?;

        state.serialize_field("pages", &serialized_pages)?;

        // Serialize nodes by converting them into a vector of serializable representations
        let serializable_nodes: Vec<_> = self
            .nodes
            .iter()
            .map(|(id, node)| {
                let node_data = serde_json::to_value(node).map_err(serde::ser::Error::custom)?;
                Ok(node_data)
            })
            .collect::<Result<_, _>>()?;
        state.serialize_field("nodes", &serializable_nodes)?;

        state.end()
    }
}

// Implement Serialize for dyn Node to allow serialization of concrete types
impl Serialize for dyn Node {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.get_node_type() {
            NodeType::Page => {
                if let Some(page) = self.as_any().downcast_ref::<Page>() {
                    page.serialize(serializer)
                } else {
                    Err(serde::ser::Error::custom("Failed to downcast to Page"))
                }
            }
            NodeType::Line => {
                if let Some(line) = self.as_any().downcast_ref::<Line>() {
                    line.serialize(serializer)
                } else {
                    Err(serde::ser::Error::custom("Failed to downcast to Line"))
                }
            }
            NodeType::Arc => {
                if let Some(arc) = self.as_any().downcast_ref::<Arc>() {
                    arc.serialize(serializer)
                } else {
                    Err(serde::ser::Error::custom("Failed to downcast to Arc"))
                }
            }
        }
    }
}

// impl Debug for dyn Command {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         self.fmt(f)
//     }
// }

trait Command: std::fmt::Debug {
    fn execute(&self, dm: &mut DataModel);
    fn undo(&self, dm: &mut DataModel);
    // fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //     write!(f, "Command")
    // }
}

#[derive(Debug)]
struct CreatePageCommand {
    id: String,
    name: String,
    description: String,
}
impl Command for CreatePageCommand {
    fn execute(&self, dm: &mut DataModel) {
        let page = Page::new(self.id.clone(), self.name.clone(), self.description.clone());
        dm.pages.insert(self.id.clone(), page);
    }

    fn undo(&self, dm: &mut DataModel) {
        dm.pages.remove(&self.id);
    }
}
impl CreatePageCommand {
    fn new(id: String, name: String, description: String) -> Self {
        CreatePageCommand {
            id,
            name,
            description,
        }
    }
}

// impl Display for CreatePageCommand {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "CreatePageCommand: id={}, name={}, description={}",
//             self.id, self.name, self.description
//         )
//     }
// }

#[derive(Debug)]
struct CreateLineCommand {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
}
impl Command for CreateLineCommand {
    fn execute(&self, dm: &mut DataModel) {
        let id = dm.id_counter.next();
        let line = Line::new(id.clone(), self.x1, self.y1, self.x2, self.y2);
        dm.nodes.insert(id, Box::new(line));
    }

    fn undo(&self, dm: &mut DataModel) {
        // Undo logic for creating a line
    }
}
impl CreateLineCommand {
    fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        CreateLineCommand { x1, y1, x2, y2 }
    }
}

#[derive(Debug)]
struct CreateArcCommand {
    x: f64,
    y: f64,
    r: f64,
    angle_start: f64,
    angle_end: f64,
}
impl Command for CreateArcCommand {
    fn execute(&self, dm: &mut DataModel) {
        let id = dm.id_counter.next();
        let arc = Arc::new(
            id.clone(),
            self.x,
            self.y,
            self.r,
            self.angle_start,
            self.angle_end,
        );
        dm.nodes.insert(id, Box::new(arc));
    }

    fn undo(&self, dm: &mut DataModel) {
        // Undo logic for creating an arc
    }
}
impl CreateArcCommand {
    fn new(x: f64, y: f64, r: f64, angle_start: f64, angle_end: f64) -> Self {
        CreateArcCommand {
            x,
            y,
            r,
            angle_start,
            angle_end,
        }
    }
}

fn save_data(dm: &DataModel) {
    println!("Saving data ...");

    let serialized = serde_json::to_string(dm).unwrap();
    println!("Serialized: {}", serialized);

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
                dm.id_counter.next(),
                parameter.next().unwrap_or("new-page").to_string(),
                parameter.next().unwrap_or("page-description").to_string(),
            );

            cmd.execute(dm);
            dm.add_command(Box::new(cmd));
        }
        NodeType::Line => {
            let x1: f64 = parameter.next().unwrap_or("0.0").parse().unwrap_or(0.0);
            let y1: f64 = parameter.next().unwrap_or("0.0").parse().unwrap_or(0.0);
            let x2: f64 = parameter.next().unwrap_or("0.0").parse().unwrap_or(0.0);
            let y2: f64 = parameter.next().unwrap_or("0.0").parse().unwrap_or(0.0);

            let cmd = CreateLineCommand::new(x1, y1, x2, y2);
            cmd.execute(dm);
            dm.add_command(Box::new(cmd));
        }

        NodeType::Arc => {
            let x: f64 = parameter.next().unwrap_or("0.0").parse().unwrap_or(0.0);
            let y: f64 = parameter.next().unwrap_or("0.0").parse().unwrap_or(0.0);
            let r: f64 = parameter.next().unwrap_or("0.0").parse().unwrap_or(0.0);
            let angle_start: f64 = parameter.next().unwrap_or("0.0").parse().unwrap_or(0.0);
            let angle_end: f64 = parameter.next().unwrap_or("360.0").parse().unwrap_or(0.0);

            let cmd = CreateArcCommand::new(x, y, r, angle_start, angle_end);
            cmd.execute(dm);
            dm.add_command(Box::new(cmd));
        }
        _ => {
            eprintln!("Error: Unknown node type: {}", node_type);
        }
    }
}

fn execute_list(dm: &DataModel, parameter: &mut SplitWhitespace<'_>) {
    let node_type = parameter.next().unwrap_or("").to_string();

    match node_type.as_str() {
        "pages" => {
            for page in dm.pages.values() {
                println!("Page ID: {}, Name: {}", page.get_id(), page.name);
            }
        }
        "page" => {
            let id = parameter.next().unwrap_or("").to_string();
            if let Some(page) = dm.pages.get(&id) {
                println!("Page ID: {}, Name: {}", page.get_id(), page.name);
            } else {
                eprintln!("Error: Page with ID {} not found", id);
            }
        }
        "lines" => {
            for node in dm.nodes.values() {
                match node.get_node_type() {
                    NodeType::Line => {
                        if let Some(line) = node.as_any().downcast_ref::<Line>() {
                            println!(
                                "Line ID: {}, Coordinates: ({}, {}), ({}, {})",
                                line.get_id(),
                                line.x1,
                                line.y1,
                                line.x2,
                                line.y2
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
                    "undo" => {
                        // undo from the dm.undo_stack
                        if let Some(cmd) = data_model.undo_stack.pop() {
                            cmd.undo(&mut data_model);
                            println!("Undoing last command: {:?}", cmd);
                        } else {
                            println!("No commands to undo");
                        }
                    }
                    // undo(&mut data_model),
                    _ => {
                        eprintln!("Error: Unknown command: {}", command_name);
                    }
                }
            }
            Err(error) => eprintln!("Error reading line: {}", error),
        }
    }
}

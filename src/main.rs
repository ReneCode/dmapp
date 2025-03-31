use core::f64;
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
    content: String,
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
            content,
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
}

impl Default for DataModel {
    fn default() -> Self {
        DataModel {
            id_counter: IdCounter::default(),
            pages: HashMap::new(),
            nodes: HashMap::new(),
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

fn create_page(dm: &mut DataModel, name: &str) {
    let id = dm.id_counter.next();

    let page = Page {
        node_type: NodeType::Page,
        id: id.to_string(),
        name: name.to_string(),
        content: "".to_string(),
    };

    println!("Creating page: {:?}", page);
    dm.pages.insert(id.to_string(), page);
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
            let name = parameter.next().unwrap_or("").to_string();
            create_page(dm, &name);
        }
        NodeType::Line => {
            let id = dm.id_counter.next();

            let x1: f64 = parameter.next().unwrap_or("0.0").parse().unwrap_or(0.0);
            let y1: f64 = parameter.next().unwrap_or("0.0").parse().unwrap_or(0.0);
            let x2: f64 = parameter.next().unwrap_or("0.0").parse().unwrap_or(0.0);
            let y2: f64 = parameter.next().unwrap_or("0.0").parse().unwrap_or(0.0);

            let line = Line::new(id.clone(), x1, y1, x2, y2);
            dm.nodes.insert(id, Box::new(line));
        }

        NodeType::Arc => {
            let id = dm.id_counter.next();

            let x: f64 = parameter.next().unwrap_or("0.0").parse().unwrap_or(0.0);
            let y: f64 = parameter.next().unwrap_or("0.0").parse().unwrap_or(0.0);
            let r: f64 = parameter.next().unwrap_or("0.0").parse().unwrap_or(0.0);
            let angle_start: f64 = parameter.next().unwrap_or("0.0").parse().unwrap_or(0.0);
            let angle_end: f64 = parameter.next().unwrap_or("360.0").parse().unwrap_or(0.0);

            let arc = Arc::new(id.clone(), x, y, r, angle_start, angle_end);
            dm.nodes.insert(id, Box::new(arc));
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
                    _ => {
                        eprintln!("Error: Unknown command: {}", command_name);
                    }
                }
            }
            Err(error) => eprintln!("Error reading line: {}", error),
        }
    }
}

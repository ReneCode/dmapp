use std::{
    collections::HashMap,
    fs::File,
    io::Write,
    io::{self, BufRead},
    str::SplitWhitespace,
};

use serde::{ser::SerializeMap, Deserialize, Serialize, Serializer};

mod commandparser;

#[derive(Debug, Serialize, Deserialize)]
enum NodeType {
    // Root,
    Page,
}

trait Node: std::fmt::Debug {
    fn get_id(&self) -> &str;
    fn get_node_type(&self) -> &NodeType;
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

#[derive(Debug, Serialize)]
struct DataModel {
    pages: HashMap<String, Page>,

    #[serde(skip_serializing)]
    id_counter: IdCounter,
}

impl Default for DataModel {
    fn default() -> Self {
        DataModel {
            id_counter: IdCounter::default(),
            pages: HashMap::new(),
        }
    }
}

// impl Serialize for DataModel {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         println!("Serializing DataModel ....");
//         let mut seq = serializer.serialize_map(Some(self.nodes.len()))?;
//         for (key, value) in &self.nodes {
//             // seq.serialize_entry(key, value)?;
//             // seq.serialize_entry(key, value)?;
//             //     map.serialize_entry(key, value)?;
//         }
//         seq.end()

//         // serializer..serialize_int(self.id_counter.counter)

//         // .end()
//     }
// }

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
    let node_type = parameter.next().unwrap_or("").to_string();

    match node_type.as_str() {
        "page" => {
            let name = parameter.next().unwrap_or("").to_string();
            create_page(dm, &name);
        }
        _ => {
            eprintln!("Error: Unknown node type: {}", node_type);
        }
    }

    // let node_type = parameter.get(0).unwrap_or(&"".to_string());
    // let para = parameter.get(1).unwrap_or(&"".to_string());

    // match parameter.get(0) {
    //     Some("page") => {
    //         create_page(dm, para);
    //     }
    //     "page" => {
    //         create_page(dm, para);
    //     }
    //     _ => {
    //         eprintln!("Error: Unknown node type: {}", node_type);
    //     }
    // }
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
                    "save" => save_data(&data_model),
                    _ => {
                        eprintln!("Error: Unknown command: {}", command_name);
                    }
                }

                // let mut words = line.split_whitespace();
                // match words.next() {
                //     None => {
                //         eprintln!("Error: Missing command")
                //     }
                //     Some("create") => {
                //         let name = match words.next() {
                //             Some("page") => {
                //                 println!("Creating page");
                //                 create_page(dm, name);
                //             }
                //             Some(_) => {
                //                 eprintln!("Error: create, unknown type");
                //                 continue;
                //             }
                //             None => {
                //                 eprintln!("Error: create, missing parameter");
                //             }
                //         };
                //     }
                //     Some(cmd) => {
                //         eprintln!("Error: Unknown command: {cmd}");
                //     }
                // }
            }
            Err(error) => eprintln!("Error reading line: {}", error),
        }
    }
}

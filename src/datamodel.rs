use itertools::Itertools;
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::command::Command;
use crate::node::Node;
use crate::page::Page;

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
pub struct DataModel {
    pages: HashMap<String, Page>,
    nodes: HashMap<String, Box<dyn Node>>,
    // #[serde(skip_serializing)]
    id_counter: IdCounter,
    undo_stack: Vec<Box<dyn Command>>,
}
impl DataModel {
    pub fn add_command(&mut self, cmd: Box<dyn Command>) {
        self.undo_stack.push(cmd);
    }

    pub fn next_id(&mut self) -> String {
        self.id_counter.next()
    }

    pub fn insert_page(&mut self, page: Page) {
        self.pages.insert(page.get_id().to_string(), page);
    }
    pub fn remove_page(&mut self, id: &str) {
        self.pages.remove(id);
    }

    pub fn insert_node(&mut self, node: Box<dyn Node>) {
        self.nodes.insert(node.get_id().to_string(), node);
    }
    pub fn remove_node(&mut self, id: &str) {
        self.nodes.remove(id);
    }

    pub fn get_pages(&self) -> Vec<&Page> {
        let result = self.pages.values().collect_vec();
        result
    }
    pub fn get_page(&self, id: &str) -> Option<&Page> {
        self.pages.get(id)
    }
    pub fn get_nodes(&self) -> Vec<&Box<dyn Node>> {
        let result = self.nodes.values().collect_vec();
        result
    }

    pub fn undo_pop(&mut self) -> Option<Box<dyn Command>> {
        if let Some(cmd) = self.undo_stack.pop() {
            cmd.undo(self);
            Some(cmd)
        } else {
            None
        }
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

//

use serde::{Deserialize, Serialize};

use crate::node::{Node, NodeType};

#[derive(Debug, Serialize, Deserialize)]
pub struct Line {
    node_type: NodeType,
    id: String,
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
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
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Line {
    pub fn new(id: String) -> Self {
        Line {
            node_type: NodeType::Line,
            id,
            x1: 0.0,
            y1: 0.0,
            x2: 50.0,
            y2: 50.0,
        }
    }
    pub fn get_x1(&self) -> f64 {
        self.x1
    }
    pub fn get_y1(&self) -> f64 {
        self.y1
    }
    pub fn get_x2(&self) -> f64 {
        self.x2
    }
    pub fn get_y2(&self) -> f64 {
        self.y2
    }
}

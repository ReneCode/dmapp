use serde::{Deserialize, Serialize};

use crate::node::{Node, NodeType};

#[derive(Debug, Serialize, Deserialize)]
pub struct Arc {
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
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Arc {
    pub fn new(id: String, x: f64, y: f64, r: f64, angle_start: f64, angle_end: f64) -> Self {
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

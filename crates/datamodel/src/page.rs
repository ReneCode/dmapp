use serde::{Deserialize, Serialize};

use crate::node::{Node, NodeType};

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
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

impl Page {
    pub fn new(id: String, name: String, content: String) -> Self {
        Page {
            node_type: NodeType::Page,
            id,
            name,
            description: content,
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::arc::Arc;
use crate::line::Line;
use crate::page::Page;

#[derive(Debug, Serialize, Deserialize)]
pub enum NodeType {
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

// ------------------------

pub trait Node: std::fmt::Debug {
    fn get_id(&self) -> &str;
    fn get_node_type(&self) -> &NodeType;
    fn as_any(&self) -> &dyn std::any::Any;
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

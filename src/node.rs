use serde::{Deserialize, Serialize};
use crate::operations::Operation;

#[derive(Debug, Serialize, Deserialize)]
pub enum NodeType {
    Component(usize),
    Lambda(usize),
    Base(Operation),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub inputs: Vec<usize>,
    pub outputs: Vec<usize>,
    pub node_type: NodeType,
}
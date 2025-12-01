use serde::{Deserialize, Serialize};
use crate::operations::Operation;

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum NodeType {
    Component(usize),
    Lambda(usize),
    Base(Operation),
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Node {
    pub inputs: Vec<usize>,
    pub outputs: Vec<usize>,
    pub node_type: NodeType,
}
use serde::{Deserialize, Serialize};
use crate::constraint::Constraint;
use crate::node::Node;
use crate::wire::Wire;

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Component {
    pub wires: Vec<Wire>,
    pub inputs: Vec<usize>,
    pub outputs: Vec<usize>,
    pub nodes: Vec<Node>,
    pub constraints: Vec<Constraint>
}
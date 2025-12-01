use serde::{Deserialize, Serialize};
use crate::operations::Operation;

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Constraint {
    pub signals: Vec<usize>,
    pub operation: Operation,
}
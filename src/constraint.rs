use serde::{Deserialize, Serialize};
use crate::operations::Operation;

#[derive(Debug, Serialize, Deserialize)]
pub struct Constraint {
    pub signals: Vec<usize>,
    pub operation: Operation,
}
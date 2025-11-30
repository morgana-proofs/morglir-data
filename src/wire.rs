use serde::{Deserialize, Serialize};
use crate::types::Type;

#[derive(Debug, Serialize, Deserialize)]
pub struct Wire {
    pub wire_type: Type,
    pub is_sig: bool,
}
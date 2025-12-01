use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::component::Component;

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Circuit {
    pub lambdas: Vec<String>,
    pub op_data: Vec<Value>,
    pub components: Vec<Component>,
}
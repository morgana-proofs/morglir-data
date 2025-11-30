use serde::{Deserialize, Serialize};
use crate::component::Component;

#[derive(Debug, Serialize, Deserialize)]
pub struct Circuit {
    pub lambdas: Vec<String>,
    pub components: Vec<Component>,
}
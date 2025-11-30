use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    Field,
    U32,
    Pointer(Box<Type>),
    Tuple(Vec<Type>),
}
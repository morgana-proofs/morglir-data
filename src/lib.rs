mod circuit;
mod component;
mod types;
mod wire;
mod node;
mod operations;
mod constraint;

pub mod prelude {
    pub use crate::{
        circuit::Circuit,
        component::Component,
        types::Type,
        wire::Wire,
        node::{Node, NodeType},
        operations::Operation,
        constraint::Constraint,
    };
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn some_circuit() {
        Circuit {
            lambdas: vec![
                "$0.invert()".to_string(),
            ],
            components: vec![
                Component {  // c := a / b 
                    wires: vec![
                        Wire {  // a 
                            wire_type: Type::Field,
                            is_sig: true,
                        },
                        Wire {  // b
                            wire_type: Type::Field,
                            is_sig: true,
                        },
                        Wire {  // c
                            wire_type: Type::Field,
                            is_sig: true,
                        },
                        Wire {  // d := 1 / b
                            wire_type: Type::Field,
                            is_sig: true,
                        },
                    ],
                    inputs: vec![
                        0,
                        1,
                    ],
                    outputs: vec![
                        2,
                    ],
                    nodes: vec![
                        Node {  // compute d
                            inputs: vec![1],
                            outputs: vec![3],
                            node_type: NodeType::Lambda(0),
                        },
                        Node {  // compute c
                            inputs: vec![0, 3],
                            outputs: vec![2],
                            node_type: NodeType::Base(Operation::Mul__Field_Field__Field),
                        },
                    ],
                    constraints: vec![
                        Constraint {
                            signals: vec![1, 3, todo!()],
                            operation: Operation::Mul__Field_Field__Field,
                        },
                        Constraint {  // check c
                            signals: vec![0, 3, 2],
                            operation: Operation::Mul__Field_Field__Field,
                        }
                    ],
                }
            ],
        };
    }
}
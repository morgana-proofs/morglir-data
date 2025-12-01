mod circuit;
mod component;
mod types;
mod wire;
mod node;
mod operations;
mod constraint;
mod program;

pub mod prelude {
    pub use crate::{
        circuit::Circuit,
        component::Component,
        types::Type,
        wire::Wire,
        node::{Node, NodeType},
        operations::{Operation, OperationType},
        constraint::Constraint,
        program::{Program, Metadata},
    };
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use serde_json::json;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn some_circuit() {
        let some_program = Program (
            Metadata::new(),
            Circuit {
                lambdas: vec![
                    "$0.invert()".to_string(),
                ],
                op_data: vec![
                    json!({
                        "field_const_from": "1",
                    }),
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
                            Wire {  // 1
                                wire_type: Type::Field,
                                is_sig: true,
                            },
                            Wire {  // d := 1 / b
                                wire_type: Type::Field,
                                is_sig: true,
                            },
                            Wire {  // c
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
                            Node {  // set 1
                                inputs: vec![],
                                outputs: vec![2],
                                node_type: NodeType::Base(Operation {
                                    operation_type: OperationType::Const____Field,
                                    data_link: Some(0),
                                }),
                            },
                            Node {  // compute d
                                inputs: vec![1],
                                outputs: vec![3],
                                node_type: NodeType::Lambda(0),
                            },
                            Node {  // compute c
                                inputs: vec![0, 3],
                                outputs: vec![4],
                                node_type: NodeType::Base(Operation {
                                    operation_type: OperationType::Mul__Field_Field__Field,
                                    data_link: None,
                                }),
                            },
                        ],
                        constraints: vec![
                            Constraint {  // check d
                                signals: vec![1, 3, 2],
                                operation: Operation {
                                    operation_type: OperationType::Mul__Field_Field__Field,
                                    data_link: None,
                                },
                            },
                            Constraint {  // check c
                                signals: vec![0, 3, 4],
                                operation: Operation {
                                    operation_type: OperationType::Mul__Field_Field__Field,
                                    data_link: None,
                                },
                            }
                        ],
                    }
                ],
            }
        );

        some_program.to_writer(
            File::options().create(true).write(true).open("samples/division.morgl").unwrap()
        ).unwrap();
        let reloaded_program = Program::from_reader(
            File::open("samples/division.morgl").unwrap(),
        ).unwrap();
        assert_eq!(some_program, reloaded_program);
    }
}
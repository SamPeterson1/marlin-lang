use serde::Serialize;

use crate::{ast::{ASTNode, ASTWrapper}, token::{Position, PositionRange}};

#[derive(Serialize)]
pub struct Program {
    pub items: Vec<Box<dyn ASTNode>>
}

impl ASTWrapper<Program> {
    pub fn new_program(items: Vec<Box<dyn ASTNode>>) -> Self {
        let position = if items.len() > 0 {
            PositionRange::concat(items[0].get_position(), items[items.len() - 1].get_position())
        } else {
            PositionRange::new(Position::new(0, 0))
        };

        ASTWrapper {
            data: Program {
                items
            },
            position
        }
    }
}
use serde::Serialize;

use crate::ast::ASTNode;
use crate::{impl_positioned};
use crate::lexer::token::PositionRange;

#[derive(Serialize)]
pub struct Arguments {
    pub args: Vec<Box<dyn ASTNode>>,
    position: PositionRange,
}

impl Arguments {
    pub fn new(arguments: Vec<Box<dyn ASTNode>>, position: PositionRange) -> Self {
        Self {
            args: arguments,
            position,
        }
    }
}

impl_positioned!(Arguments);
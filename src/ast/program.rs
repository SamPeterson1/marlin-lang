use serde::Serialize;

use crate::ast::ASTNode;
use crate::{impl_ast_node, impl_positioned};
use crate::lexer::token::PositionRange;

#[derive(Serialize)]
pub struct Program {
    pub items: Vec<Box<dyn ASTNode>>,
    position: PositionRange,
}

impl Program {
    pub fn new(items: Vec<Box<dyn ASTNode>>, position: PositionRange) -> Self {
        Self {
            items,
            position,
        }
    }
}

impl_positioned!(Program);
impl_ast_node!(Program, visit_program);
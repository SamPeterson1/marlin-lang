use serde::Serialize;

use crate::ast::{ASTNode, AstId};
use crate::{impl_ast_node, impl_positioned, new_ast_id};
use crate::lexer::token::PositionRange;

#[derive(Serialize)]
pub struct Program {
    pub items: Vec<Box<dyn ASTNode>>,
    position: PositionRange,
    id: AstId,
}

impl Program {
    pub fn new(items: Vec<Box<dyn ASTNode>>, position: PositionRange) -> Self {
        Self {
            items,
            position,
            id: new_ast_id!(),
        }
    }
}

impl_positioned!(Program);
impl_ast_node!(Program, visit_program);
use serde::Serialize;

use crate::ast::ASTNode;
use crate::{impl_ast_node, impl_positioned};
use crate::lexer::token::PositionRange;

#[derive(Serialize)]
pub struct BlockExpr {
    pub exprs: Vec<Box<dyn ASTNode>>,
    position: PositionRange,
}

impl BlockExpr {
    pub fn new(exprs: Vec<Box<dyn ASTNode>>, position: PositionRange) -> Self {
        Self {
            exprs,
            position,
        }
    }    
}

impl_positioned!(BlockExpr);
impl_ast_node!(BlockExpr, visit_block);
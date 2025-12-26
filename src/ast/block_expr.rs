use serde::Serialize;

use crate::ast::{ASTNode, AstId};
use crate::{impl_ast_node, impl_positioned, new_ast_id};
use crate::lexer::token::PositionRange;

#[derive(Serialize)]
pub struct BlockExpr {
    pub exprs: Vec<Box<dyn ASTNode>>,
    position: PositionRange,
    id: AstId,
}

impl BlockExpr {
    pub fn new(exprs: Vec<Box<dyn ASTNode>>, position: PositionRange) -> Self {
        Self {
            exprs,
            position,
            id: new_ast_id!(),
        }
    }    
}

impl_positioned!(BlockExpr);
impl_ast_node!(BlockExpr, visit_block);
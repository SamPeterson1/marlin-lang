use serde::Serialize;

use crate::{ast::{ASTNode, ASTWrapper}, impl_ast_node, token::PositionRange};

#[derive(Serialize)]
pub struct BlockExpr {
    pub exprs: Vec<Box<dyn ASTNode>>,
}

impl ASTWrapper<BlockExpr> {
    pub fn new_block(exprs: Vec<Box<dyn ASTNode>>, position: PositionRange) -> Self {
        ASTWrapper {
            data: BlockExpr {
                exprs
            },
            position
        }
    }    
}

impl_ast_node!(BlockExpr, visit_block);
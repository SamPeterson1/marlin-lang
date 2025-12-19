use serde::Serialize;

use crate::ast::ASTNode;
use crate::resolver::ResolvedType;
use crate::{impl_ast_node, impl_positioned, impl_typed};
use crate::lexer::token::PositionRange;

#[derive(Serialize)]
pub struct BlockExpr {
    pub exprs: Vec<Box<dyn ASTNode>>,
    position: PositionRange,
    resolved_type: Option<ResolvedType>,
}

impl BlockExpr {
    pub fn new(exprs: Vec<Box<dyn ASTNode>>, position: PositionRange) -> Self {
        Self {
            exprs,
            position,
            resolved_type: None,
        }
    }    
}

impl_positioned!(BlockExpr);
impl_typed!(BlockExpr);
impl_ast_node!(BlockExpr, visit_block);
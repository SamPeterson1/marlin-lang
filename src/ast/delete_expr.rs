use serde::Serialize;

use crate::ast::ASTNode;
use crate::{impl_ast_node, impl_positioned};
use crate::lexer::token::PositionRange;

#[derive(Serialize)]
pub struct DeleteExpr {
    pub expr: Box<dyn ASTNode>,
    position: PositionRange,
}

impl DeleteExpr {
    pub fn new(expr: Box<dyn ASTNode>, position: PositionRange) -> Self {
        Self {
            expr,
            position,
        } 
    }    
}

impl_positioned!(DeleteExpr);
impl_ast_node!(DeleteExpr, visit_delete);
use serde::Serialize;

use crate::ast::{ASTNode, AstId};
use crate::{impl_ast_node, impl_positioned, new_ast_id};
use crate::lexer::token::PositionRange;

#[derive(Serialize)]
pub struct DeleteExpr {
    pub expr: Box<dyn ASTNode>,
    position: PositionRange,
    id: AstId,
}

impl DeleteExpr {
    pub fn new(expr: Box<dyn ASTNode>, position: PositionRange) -> Self {
        Self {
            expr,
            position,
            id: new_ast_id!(),
        } 
    }    
}

impl_positioned!(DeleteExpr);
impl_ast_node!(DeleteExpr, visit_delete);
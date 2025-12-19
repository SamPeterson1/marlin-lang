use serde::Serialize;

use crate::ast::ASTNode;
use crate::resolver::ResolvedType;
use crate::{impl_ast_node, impl_positioned, impl_typed};
use crate::lexer::token::PositionRange;

#[derive(Serialize)]
pub struct DeleteExpr {
    pub expr: Box<dyn ASTNode>,
    position: PositionRange,
    resolved_type: Option<ResolvedType>,
}

impl DeleteExpr {
    pub fn new(expr: Box<dyn ASTNode>, position: PositionRange) -> Self {
        Self {
            expr,
            position,
            resolved_type: None,
        } 
    }    
}

impl_positioned!(DeleteExpr);
impl_typed!(DeleteExpr);
impl_ast_node!(DeleteExpr, visit_delete);
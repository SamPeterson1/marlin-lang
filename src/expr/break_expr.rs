use serde::Serialize;

use crate::{expr::{ASTNode, ASTWrapper}, token::PositionRange};

#[derive(Serialize)]
pub struct BreakExpr {
    pub expr: Box<dyn ASTNode>,
}

impl ASTWrapper<BreakExpr> {
    pub fn new_break(expr: Box<dyn ASTNode>, position: PositionRange) -> Self {
        ASTWrapper {
            data: BreakExpr {
                expr,
            },
            position
        }
    }
}

crate::impl_ast_node!(BreakExpr, visit_break);
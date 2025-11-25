use serde::Serialize;

use crate::{ast::{ASTNode, ASTWrapper}, token::PositionRange};

#[derive(Serialize)]
pub struct BreakExpr {
    pub expr: Option<Box<dyn ASTNode>>,
}

impl ASTWrapper<BreakExpr> {
    pub fn new_break(expr: Option<Box<dyn ASTNode>>, position: PositionRange) -> Self {
        ASTWrapper {
            data: BreakExpr {
                expr,
            },
            position
        }
    }
}

crate::impl_ast_node!(BreakExpr, visit_break);
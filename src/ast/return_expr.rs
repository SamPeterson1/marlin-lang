use serde::Serialize;

use crate::{ast::{ASTNode, ASTWrapper}, token::PositionRange};

#[derive(Serialize)]
pub struct ReturnExpr {
    pub expr: Option<Box<dyn ASTNode>>,
}

impl ASTWrapper<ReturnExpr> {
    pub fn new_return(expr: Option<Box<dyn ASTNode>>, position: PositionRange) -> Self {
        ASTWrapper {
            data: ReturnExpr {
                expr,
            },
            position
        }
    }
}

crate::impl_ast_node!(ReturnExpr, visit_return);
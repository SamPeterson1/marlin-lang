use serde::Serialize;

use crate::{expr::{ASTNode, ASTWrapper}, token::PositionRange};

#[derive(Serialize)]
pub struct PutCharExpr {
    pub expr: Box<dyn ASTNode>,
}

impl ASTWrapper<PutCharExpr> {
    pub fn new_put_char(expr: Box<dyn ASTNode>, position: PositionRange) -> Self {
        ASTWrapper {
            data: PutCharExpr {
                expr,
            },
            position
        }
        
    }
}

crate::impl_ast_node!(PutCharExpr, visit_put_char);
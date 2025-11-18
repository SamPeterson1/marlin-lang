use serde::Serialize;

use crate::{ast::{ASTNode, ASTWrapper}, token::PositionRange};

#[derive(Serialize)]
pub struct IfExpr {
    pub condition: Box<dyn ASTNode>,
    pub success: Box<dyn ASTNode>,
    pub fail: Option<Box<dyn ASTNode>>,
}

impl ASTWrapper<IfExpr> {
    pub fn new_if(condition: Box<dyn ASTNode>, success: Box<dyn ASTNode>, fail: Option<Box<dyn ASTNode>>, position: PositionRange) -> Self {
        ASTWrapper {
            data: IfExpr {
                condition,
                success,
                fail,
            },
            position
        }
        
    }
}

crate::impl_ast_node!(IfExpr, visit_if);
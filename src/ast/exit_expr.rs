use serde::Serialize;

use crate::{ast::{ASTNode, ASTWrapper}, token::PositionRange};

#[derive(Serialize, Clone)]
pub enum ExitType {
    Return,
    Result,
    Break,
}

#[derive(Serialize)]
pub struct ExitExpr {
    pub exit_type: ExitType,
    pub expr: Option<Box<dyn ASTNode>>,
}

impl ASTWrapper<ExitExpr> {
    pub fn new_exit(exit_type: ExitType, expr: Option<Box<dyn ASTNode>>, position: PositionRange) -> Self {
        ASTWrapper {
            data: ExitExpr {
                exit_type,
                expr,
            },
            position
        }
    }
}

crate::impl_ast_node!(ExitExpr, visit_exit);
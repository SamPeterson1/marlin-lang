use serde::Serialize;

use crate::ast::ASTNode;
use crate::{impl_ast_node, impl_positioned};
use crate::lexer::token::PositionRange;

#[derive(Serialize)]
pub struct IfExpr {
    pub condition: Box<dyn ASTNode>,
    pub success: Box<dyn ASTNode>,
    pub fail: Option<Box<dyn ASTNode>>,
    position: PositionRange,
}

impl IfExpr {
    pub fn new(condition: Box<dyn ASTNode>, success: Box<dyn ASTNode>, fail: Option<Box<dyn ASTNode>>, position: PositionRange) -> Self {
        Self {
            condition,
            success,
            fail,
            position,
        }
    }
}

impl_positioned!(IfExpr);
impl_ast_node!(IfExpr, visit_if);
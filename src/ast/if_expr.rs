use serde::Serialize;

use crate::ast::ASTNode;
use crate::resolver::ResolvedType;
use crate::{impl_ast_node, impl_positioned, impl_typed};
use crate::lexer::token::PositionRange;

#[derive(Serialize)]
pub struct IfExpr {
    pub condition: Box<dyn ASTNode>,
    pub success: Box<dyn ASTNode>,
    pub fail: Option<Box<dyn ASTNode>>,
    position: PositionRange,
    resolved_type: Option<ResolvedType>,
}

impl IfExpr {
    pub fn new(condition: Box<dyn ASTNode>, success: Box<dyn ASTNode>, fail: Option<Box<dyn ASTNode>>, position: PositionRange) -> Self {
        Self {
            condition,
            success,
            fail,
            position,
            resolved_type: None,
        }
    }
}

impl_positioned!(IfExpr);
impl_typed!(IfExpr);
impl_ast_node!(IfExpr, visit_if);
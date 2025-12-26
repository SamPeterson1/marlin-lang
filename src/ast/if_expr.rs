use serde::Serialize;

use crate::ast::{ASTNode, AstId};
use crate::{impl_ast_node, impl_positioned, new_ast_id};
use crate::lexer::token::PositionRange;

#[derive(Serialize)]
pub struct IfExpr {
    pub condition: Box<dyn ASTNode>,
    pub success: Box<dyn ASTNode>,
    pub fail: Option<Box<dyn ASTNode>>,
    position: PositionRange,
    id: AstId,
}

impl IfExpr {
    pub fn new(condition: Box<dyn ASTNode>, success: Box<dyn ASTNode>, fail: Option<Box<dyn ASTNode>>, position: PositionRange) -> Self {
        Self {
            condition,
            success,
            fail,
            position,
            id: new_ast_id!(),
        }
    }
}

impl_positioned!(IfExpr);
impl_ast_node!(IfExpr, visit_if);
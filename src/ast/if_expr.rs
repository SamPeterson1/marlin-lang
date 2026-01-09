use serde::Serialize;

use crate::ast::{ASTEnum, ASTNode, AstId};
use crate::compiler::visit::{Parsed, Phase};
use crate::{impl_ast_node, new_ast_id};
use crate::lexer::token::PositionRange;

#[derive(Serialize)]
pub struct IfExpr<P: Phase = Parsed> {
    pub condition: ASTEnum<P>,
    pub success: ASTEnum<P>,
    pub fail: Option<ASTEnum<P>>,
    position: PositionRange,
    id: AstId,
}

impl IfExpr {
    pub fn new(condition: ASTEnum, success: ASTEnum, fail: Option<ASTEnum>, position: PositionRange) -> Self {
        Self {
            condition,
            success,
            fail,
            position,
            id: new_ast_id!(),
        }
    }
}

impl_ast_node!(IfExpr, visit_if);
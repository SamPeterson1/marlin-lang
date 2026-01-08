use serde::Serialize;

use crate::ast::{ASTEnum, AstId};
use crate::compiler::stages::{Parsed, Phase};
use crate::{impl_ast_node, new_ast_id};
use crate::lexer::token::PositionRange;

#[derive(Serialize)]
pub struct ArrayAccess<P: Phase = Parsed> {
    pub expr: ASTEnum<P>,
    pub index: ASTEnum<P>,
    position: PositionRange,
    id: AstId,
}

impl ArrayAccess {
    pub fn new(expr: ASTEnum, index: ASTEnum, position: PositionRange) -> Self {
        Self {
            expr,
            index,
            position,
            id: new_ast_id!(),
        }
    }
}

impl_ast_node!(ArrayAccess, visit_array_access);

use serde::Serialize;

use crate::ast::{ASTEnum, ASTNode, AstId};
use crate::compiler::visit::{Parsed, Phase};
use crate::{impl_ast_node, new_ast_id};
use crate::lexer::token::{Located, PositionRange};

#[derive(Serialize, Clone, PartialEq, Eq)]
pub enum ExitType {
    Return,
    Result,
    Break,
}

#[derive(Serialize)]
pub struct ExitExpr<P: Phase = Parsed> {
    pub exit_type: ExitType,
    pub expr: Option<ASTEnum<P>>,
    pub label: Option<Located<String>>,
    position: PositionRange,
    id: AstId,
}

impl ExitExpr {
    pub fn new(exit_type: ExitType, expr: Option<ASTEnum>, label: Option<Located<String>>, position: PositionRange) -> Self {
        Self {
            exit_type,
            expr,
            label,
            position,
            id: new_ast_id!(),
        }
    }
}

impl_ast_node!(ExitExpr, visit_exit);
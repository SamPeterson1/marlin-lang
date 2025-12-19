use serde::Serialize;

use crate::ast::ASTNode;
use crate::resolver::ResolvedType;
use crate::{impl_ast_node, impl_positioned, impl_typed};
use crate::lexer::token::{Located, PositionRange};

#[derive(Serialize, Clone, PartialEq, Eq)]
pub enum ExitType {
    Return,
    Result,
    Break,
}

#[derive(Serialize)]
pub struct ExitExpr {
    pub exit_type: ExitType,
    pub expr: Option<Box<dyn ASTNode>>,
    pub label: Option<Located<String>>,
    position: PositionRange,
    resolved_type: Option<ResolvedType>,
}

impl ExitExpr {
    pub fn new(exit_type: ExitType, expr: Option<Box<dyn ASTNode>>, label: Option<Located<String>>, position: PositionRange) -> Self {
        Self {
            exit_type,
            expr,
            label,
            position,
            resolved_type: None,
        }
    }
}

impl_positioned!(ExitExpr);
impl_typed!(ExitExpr);
impl_ast_node!(ExitExpr, visit_exit);
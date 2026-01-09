use serde::Serialize;

use crate::ast::{ASTEnum, ASTNode, AstId};
use crate::compiler::visit::{Parsed, Phase};
use crate::{impl_ast_node, new_ast_id};
use crate::lexer::token::PositionRange;

#[derive(Serialize)]
pub struct AssignmentExpr<P: Phase = Parsed> {
    pub assignee: ASTEnum<P>,
    pub expr: ASTEnum<P>,
    position: PositionRange,
    id: AstId,
}

impl AssignmentExpr {
    pub fn new(assignee: ASTEnum, expr: ASTEnum, position: PositionRange) -> Self {
        Self {
            assignee,
            expr,
            position,
            id: new_ast_id!(),
        } 
    }
}

impl_ast_node!(AssignmentExpr, visit_assignment);

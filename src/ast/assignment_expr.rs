use serde::Serialize;

use crate::ast::{ASTNode, AstId};
use crate::{impl_ast_node, impl_positioned, new_ast_id};
use crate::lexer::token::PositionRange;

#[derive(Serialize)]
pub struct AssignmentExpr {
    pub assignee: Box<dyn ASTNode>,
    pub expr: Box<dyn ASTNode>,
    position: PositionRange,
    id: AstId
}

impl AssignmentExpr {
    pub fn new(assignee: Box<dyn ASTNode>, expr: Box<dyn ASTNode>, position: PositionRange) -> Self {
        Self {
            assignee,
            expr,
            position,
            id: new_ast_id!(),
        } 
    }    
}

impl_positioned!(AssignmentExpr);
impl_ast_node!(AssignmentExpr, visit_assignment);
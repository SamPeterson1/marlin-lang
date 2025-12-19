use serde::Serialize;

use crate::ast::ASTNode;
use crate::resolver::ResolvedType;
use crate::{impl_ast_node, impl_positioned, impl_typed};
use crate::lexer::token::PositionRange;

#[derive(Serialize)]
pub struct AssignmentExpr {
    pub assignee: Box<dyn ASTNode>,
    pub expr: Box<dyn ASTNode>,
    position: PositionRange,
    resolved_type: Option<ResolvedType>,
}

impl AssignmentExpr {
    pub fn new(assignee: Box<dyn ASTNode>, expr: Box<dyn ASTNode>, position: PositionRange) -> Self {
        Self {
            assignee,
            expr,
            position,
            resolved_type: None,
        } 
    }    
}

impl_positioned!(AssignmentExpr);
impl_typed!(AssignmentExpr);
impl_ast_node!(AssignmentExpr, visit_assignment);
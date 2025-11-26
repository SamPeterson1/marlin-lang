use std::rc::Rc;

use serde::Serialize;

use crate::{ast::{ASTNode, ASTWrapper, lvar_expr::LVarExpr}, token::{PositionRange, Positioned, Token}};

#[derive(Serialize)]
pub struct AssignmentExpr {
    pub assignee: ASTWrapper<LVarExpr>,
    pub expr: Box<dyn ASTNode>,
}

impl ASTWrapper<AssignmentExpr> {
    pub fn new_assignment(assignee: ASTWrapper<LVarExpr>, expr: Box<dyn ASTNode>) -> Self {
        let position = PositionRange::concat(&assignee.get_position(), expr.get_position());

        ASTWrapper {
            data: AssignmentExpr {
                assignee,
                expr
            },
            position
        }
        
    }    
}

crate::impl_ast_node!(AssignmentExpr, visit_assignment);
use std::rc::Rc;

use serde::Serialize;

use crate::{ast::{ASTNode, ASTWrapper}, token::{PositionRange, Token}};

#[derive(Serialize)]
pub struct AssignmentExpr {
    pub assignee: Rc<String>,
    pub expr: Box<dyn ASTNode>,
}

impl ASTWrapper<AssignmentExpr> {
    pub fn new_assignment(assignee: &Token, expr: Box<dyn ASTNode>) -> Self {
        let position = PositionRange::concat(&assignee.position, expr.get_position());

        ASTWrapper {
            data: AssignmentExpr {
                assignee: Rc::new(assignee.get_string().to_string()),
                expr
            },
            position
        }
        
    }    
}

crate::impl_ast_node!(AssignmentExpr, visit_assignment);
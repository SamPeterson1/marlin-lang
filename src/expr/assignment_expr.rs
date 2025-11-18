use std::rc::Rc;

use serde::Serialize;

use crate::{expr::{ASTNode, ASTWrapper, Positioned, var_expr::VarExpr}, token::PositionRange};

#[derive(Serialize)]
pub struct AssignmentExpr {
    pub assignee: Rc<ASTWrapper<VarExpr>>,
    pub expr: Box<dyn ASTNode>,
}

impl ASTWrapper<AssignmentExpr> {
    pub fn new_assignment(assignee: ASTWrapper<VarExpr>, expr: Box<dyn ASTNode>) -> Self {
        let position = PositionRange::concat(&assignee.get_position(), expr.get_position());

        ASTWrapper {
            data: AssignmentExpr {
                assignee: Rc::new(assignee),
                expr
            },
            position
        }
        
    }    
}

crate::impl_ast_node!(AssignmentExpr, visit_assignment);
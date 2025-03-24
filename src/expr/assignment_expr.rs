use std::{fmt, rc::Rc};

use crate::token::PositionRange;

use super::{var_expr::VarExpr, Expr};

pub struct AssignmentExpr {
    pub asignee: Rc<VarExpr>,
    pub expr: Box<dyn Expr>,
    pub position: PositionRange
}

impl fmt::Display for AssignmentExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"type\": \"Assignment\", \"asignee\": {}, \"expr\": {}, \"position\": \"{}\"}}", self.asignee, self.expr, self.position)
    }
}

impl AssignmentExpr {
    pub fn new(asignee: VarExpr, expr: Box<dyn Expr>) -> Box<dyn Expr> {
        let position = PositionRange::concat(asignee.get_position(), expr.get_position());

        Box::new(AssignmentExpr {
            asignee: Rc::new(asignee),
            expr,
            position
        })
    }    
}

crate::impl_expr!(AssignmentExpr, visit_assignment);
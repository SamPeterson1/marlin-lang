use std::fmt;

use crate::token::PositionRange;

use super::Expr;

pub struct BreakExpr {
    pub expr: Box<dyn Expr>,
    pub position: PositionRange
}

impl fmt::Display for BreakExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"type\": \"Break\", \"expr\": {}, \"position\": \"{}\"}}", self.expr, self.position)
    }
}

impl BreakExpr {
    pub fn new(expr: Box<dyn Expr>, position: PositionRange) -> Box<dyn Expr> {
        Box::new(BreakExpr {
            expr,
            position
        })
    }
}

crate::impl_expr!(BreakExpr, visit_break);
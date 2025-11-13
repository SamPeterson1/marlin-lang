use std::fmt;

use crate::token::PositionRange;

use super::Expr;

pub struct PutCharExpr {
    pub expr: Box<dyn Expr>,
    pub position: PositionRange
}

impl fmt::Display for PutCharExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"type\": \"PutChar\", \"expr\": {}, \"position\": \"{}\"}}", self.expr, self.position)
    }
}

impl PutCharExpr {
    pub fn new(expr: Box<dyn Expr>, position: PositionRange) -> PutCharExpr {
        PutCharExpr {
            expr,
            position
        }
    }
}

crate::impl_expr!(PutCharExpr, visit_put_char);
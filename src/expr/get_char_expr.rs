use std::fmt;

use crate::token::PositionRange;

use super::Expr;

#[derive(Debug)]
pub struct GetCharExpr {
    pub position: PositionRange
}

impl fmt::Display for GetCharExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"type\": \"GetChar\", \"position\": \"{}\"}}", self.position)
    }
}

impl GetCharExpr {
    pub fn new(position: PositionRange) -> Box<dyn Expr> {
        Box::new(GetCharExpr {
            position
        })
    }
}

crate::impl_expr!(GetCharExpr, visit_get_char);
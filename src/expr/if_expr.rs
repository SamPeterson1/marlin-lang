use std::fmt;

use crate::token::PositionRange;

use super::Expr;

pub struct IfExpr {
    pub condition: Box<dyn Expr>,
    pub success: Box<dyn Expr>,
    pub fail: Option<Box<dyn Expr>>,
    pub position: PositionRange
}

impl fmt::Display for IfExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"type\": \"If\", \"condition\": {}, \"success\": {},", self.condition, self.success)?;

        if let Some(fail) = &self.fail {
            write!(f, " \"fail\": {},", fail)?;
        }

        write!(f, " \"position\": \"{}\"}}", self.position)
    }
}

impl IfExpr {
    pub fn new(condition: Box<dyn Expr>, success: Box<dyn Expr>, fail: Option<Box<dyn Expr>>, position: PositionRange) -> Box<dyn Expr> {
        Box::new(IfExpr {
            condition,
            success,
            fail,
            position
        })
    }
}

crate::impl_expr!(IfExpr, visit_if);
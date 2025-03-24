use std::fmt;

use crate::token::PositionRange;

use super::Expr;

pub struct BlockExpr {
    pub exprs: Vec<Box<dyn Expr>>,
    pub position: PositionRange
}

impl fmt::Display for BlockExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"type\": \"Block\", \"exprs\": [")?;

        for (i, expr) in self.exprs.iter().enumerate() {
            write!(f, "{}", expr)?;

            if i + 1 < self.exprs.len() {
                write!(f, ", ")?;
            }
        }

        write!(f, "], \"position\": \"{}\"}}", self.position)
    }
}

impl BlockExpr {
    pub fn new(exprs: Vec<Box<dyn Expr>>, position: PositionRange) -> Box<dyn Expr> {
        Box::new(BlockExpr {
            exprs,
            position
        })
    }    
}

crate::impl_expr!(BlockExpr, visit_block);
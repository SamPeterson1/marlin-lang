use std::fmt;

use crate::token::{Position, PositionRange};

use super::Expr;

pub struct CallExpr {
    pub function: String,
    pub args: Vec<Box<dyn Expr>>,
    pub position: PositionRange
}

impl fmt::Display for CallExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"type\": \"Call\", \"function\": \"{}\", \"args\": [", self.function)?;

        for (i, arg) in self.args.iter().enumerate() {
            write!(f, "{}", arg)?;

            if i + 1 < self.args.len() {
                write!(f, ", ")?;
            }
        }

        write!(f, "], \"position\": \"{}\"}}", self.position)
    }
}

impl CallExpr {
    pub fn new(function: String, args: Vec<Box<dyn Expr>>) -> CallExpr {
        let position = PositionRange::new(Position::new(0, 0));

        CallExpr {
            function,
            args,
            position
        }
    }
}

crate::impl_expr!(CallExpr, visit_call);
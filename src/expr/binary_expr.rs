use std::fmt;

use crate::{operator::{self, BinaryOperator}, token::{PositionRange, TokenType}};

use super::Expr;

pub struct BinaryExpr {
    pub left: Box<dyn Expr>,
    pub right: Box<dyn Expr>,
    pub operator: Box<dyn BinaryOperator>,
    pub position: PositionRange
}

impl fmt::Display for BinaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"type\": \"Binary\", \"left\": {}, \"right\": {}, \"operator\": \"{}\", \"position\": \"{}\"}}", self.left, self.right, self.operator, self.position)
    }
}

impl BinaryExpr {
    pub fn new(left: Box<dyn Expr>, right: Box<dyn Expr>, operator_token: TokenType) -> Box<dyn Expr> {
        let operator = operator::as_binary_operator(operator_token);

        let position = PositionRange::concat(left.get_position(), right.get_position());

        Box::new(BinaryExpr {
            left,
            right,
            operator,
            position
        })
    }
}

crate::impl_expr!(BinaryExpr, visit_binary);
use std::fmt;

use crate::{operator::{self, UnaryOperator}, token::{PositionRange, Token}};

use super::Expr;

pub struct UnaryExpr {
    pub expr: Box<dyn Expr>,
    pub operator: Box<dyn UnaryOperator>,
    pub position: PositionRange
}

impl fmt::Display for UnaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"type\": \"Unary\", \"expr\": {}, \"operator\": \"{}\", \"position\": \"{}\"}}", self.expr, self.operator, self.position)
    }
}

impl UnaryExpr {
    pub fn new(expr: Box<dyn Expr>, operator_token: Token) -> Box<dyn Expr> {
        let operator = operator::as_unary_operator(operator_token.token_type);

        let position = PositionRange::concat(expr.get_position(), &operator_token.position);

        Box::new(UnaryExpr {
            expr,
            operator,
            position
        })
    }
}

crate::impl_expr!(UnaryExpr, visit_unary);

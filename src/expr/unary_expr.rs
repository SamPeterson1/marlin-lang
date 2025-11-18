use serde::Serialize;

use crate::{expr::{ASTNode, ASTWrapper}, operator::{self, UnaryOperator}, token::{PositionRange, Token}};

#[derive(Serialize)]
pub struct UnaryExpr {
    pub expr: Box<dyn ASTNode>,
    pub operator: Box<dyn UnaryOperator>,
}

impl ASTWrapper<UnaryExpr> {
    pub fn new_unary(expr: Box<dyn ASTNode>, operator_token: Token) -> Self {
        let operator = operator::as_unary_operator(operator_token.token_type);

        let position = PositionRange::concat(expr.get_position(), &operator_token.position);

        ASTWrapper {
            data: UnaryExpr {
                expr,
                operator,
            },
            position
        }
        
    }
}

crate::impl_ast_node!(UnaryExpr, visit_unary);

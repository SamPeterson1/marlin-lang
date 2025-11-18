use serde::Serialize;

use crate::{ast::{ASTNode, ASTWrapper}, operator::{self, BinaryOperator}, token::{PositionRange, TokenType}};

#[derive(Serialize)]
pub struct BinaryExpr {
    pub left: Box<dyn ASTNode>,
    pub right: Box<dyn ASTNode>,
    pub operator: Box<dyn BinaryOperator>,
}

impl ASTWrapper<BinaryExpr> {
    pub fn new_binary(left: Box<dyn ASTNode>, right: Box<dyn ASTNode>, operator_token: TokenType) -> Self {
        let operator = operator::as_binary_operator(operator_token);
        let position = PositionRange::concat(left.get_position(), right.get_position());

        ASTWrapper {
            data: BinaryExpr {
                left,
                right,
                operator,
            },
            position
        }
    }
}

crate::impl_ast_node!(BinaryExpr, visit_binary);
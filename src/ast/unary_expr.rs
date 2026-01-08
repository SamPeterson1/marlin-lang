use std::fmt::Display;

use serde::Serialize;

use crate::ast::{ASTEnum, ASTNode, AstId};
use crate::compiler::stages::{Parsed, Phase};
use crate::{impl_ast_node, new_ast_id};
use crate::lexer::token::{PositionRange, TokenType};

#[derive(Serialize, Clone, Copy)]
pub enum UnaryOperator {
    Deref,
    AddressOf,
    Not,
    Negative,
    BitwiseNot,
}

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op_str = match self {
            UnaryOperator::Deref => "*",
            UnaryOperator::AddressOf => "&",
            UnaryOperator::Not => "!",
            UnaryOperator::Negative => "-",
            UnaryOperator::BitwiseNot => "~",
        };
        write!(f, "{}", op_str)
    }
}

impl TryFrom<TokenType> for UnaryOperator {
    type Error = String;

    fn try_from(token_type: TokenType) -> Result<Self, String> {
        match token_type {
            TokenType::Star => Ok(UnaryOperator::Deref),
            TokenType::Ampersand => Ok(UnaryOperator::AddressOf),
            TokenType::Not => Ok(UnaryOperator::Not),
            TokenType::Minus => Ok(UnaryOperator::Negative),
            TokenType::Tilda => Ok(UnaryOperator::BitwiseNot),
            _ => Err(format!("Invalid token for binary operator: {:?}", token_type)),
        }
    }
}

#[derive(Serialize)]
pub struct UnaryExpr<P: Phase = Parsed> {
    pub expr: ASTEnum<P>,
    pub operator: UnaryOperator,
    position: PositionRange,
    id: AstId,
}

impl UnaryExpr {
    pub fn new(expr: ASTEnum, operator: UnaryOperator, position: PositionRange) -> Self {
        Self {
            expr,
            operator,
            position,
            id: new_ast_id!(),
        }
    }
}

impl_ast_node!(UnaryExpr, visit_unary);

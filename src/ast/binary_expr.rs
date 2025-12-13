use serde::Serialize;

use crate::ast::ASTNode;
use crate::{impl_ast_node, impl_positioned};
use crate::lexer::token::{PositionRange, TokenType};

#[derive(Serialize)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Times,
    Divide,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Equal,
    NotEqual,
    And,
    Or,
}

impl TryFrom<TokenType> for BinaryOperator {
    type Error = String;

    fn try_from(value: TokenType) -> Result<Self, Self::Error> {
        match value {
            TokenType::Plus => Ok(BinaryOperator::Plus),
            TokenType::Minus => Ok(BinaryOperator::Minus),
            TokenType::Star => Ok(BinaryOperator::Times),
            TokenType::Slash => Ok(BinaryOperator::Divide),
            TokenType::Greater => Ok(BinaryOperator::Greater),
            TokenType::GreaterEqual => Ok(BinaryOperator::GreaterEqual),
            TokenType::Less => Ok(BinaryOperator::Less),
            TokenType::LessEqual => Ok(BinaryOperator::LessEqual),
            TokenType::Equal => Ok(BinaryOperator::Equal),
            TokenType::NotEqual => Ok(BinaryOperator::NotEqual),
            TokenType::And => Ok(BinaryOperator::And),
            TokenType::Or => Ok(BinaryOperator::Or),
            _ => Err(format!("Invalid token for binary operator: {:?}", value)),
        }
    }
}

#[derive(Serialize)]
pub struct BinaryExpr {
    pub left: Box<dyn ASTNode>,
    pub right: Box<dyn ASTNode>,
    pub operator: BinaryOperator,
    position: PositionRange,
}

impl BinaryExpr {
    pub fn new(left: Box<dyn ASTNode>, right: Box<dyn ASTNode>, operator: BinaryOperator) -> Self {
        let position = PositionRange::concat(&left.get_position(), &right.get_position());

        Self {
            left,
            right,
            operator,
            position,
        }
    }
}

impl_positioned!(BinaryExpr);
impl_ast_node!(BinaryExpr, visit_binary);
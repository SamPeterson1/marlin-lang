use std::fmt::Display;

use serde::Serialize;

use crate::ast::{ASTEnum, ASTNode, AstId};
use crate::compiler::visit::{Parsed, Phase};
use crate::{impl_ast_node, new_ast_id};
use crate::lexer::token::{PositionRange, Positioned, TokenType};

#[derive(Serialize, Clone, Copy, Debug)]
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
    BitwiseOr,
    BitwiseAnd,
    BitwiseXor,
    Modulo,
    LeftShift,
    RightShift,
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op_str = match self {
            BinaryOperator::Plus => "+",
            BinaryOperator::Minus => "-",
            BinaryOperator::Times => "*",
            BinaryOperator::Divide => "/",
            BinaryOperator::Greater => ">",
            BinaryOperator::GreaterEqual => ">=",
            BinaryOperator::Less => "<",
            BinaryOperator::LessEqual => "<=",
            BinaryOperator::Equal => "==",
            BinaryOperator::NotEqual => "!=",
            BinaryOperator::And => "&&",
            BinaryOperator::Or => "||",
            BinaryOperator::BitwiseOr => "|",
            BinaryOperator::BitwiseAnd => "&",
            BinaryOperator::BitwiseXor => "^",
            BinaryOperator::Modulo => "%",
            BinaryOperator::LeftShift => "<<",
            BinaryOperator::RightShift => ">>",
        };
        write!(f, "{}", op_str)
    }
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
            TokenType::Bar => Ok(BinaryOperator::BitwiseOr),
            TokenType::Ampersand => Ok(BinaryOperator::BitwiseAnd),
            TokenType::Carat => Ok(BinaryOperator::BitwiseXor),
            TokenType::Percentage => Ok(BinaryOperator::Modulo),
            TokenType::LeftShift => Ok(BinaryOperator::LeftShift),
            TokenType::RightShift => Ok(BinaryOperator::RightShift),
            _ => Err(format!("Invalid token for binary operator: {:?}", value)),
        }
    }
}

#[derive(Serialize)]
pub struct BinaryExpr<P: Phase = Parsed> {
    pub left: ASTEnum<P>,
    pub right: ASTEnum<P>,
    pub operator: BinaryOperator,
    position: PositionRange,
    id: AstId,
}

impl BinaryExpr {
    pub fn new(left: ASTEnum, right: ASTEnum, operator: BinaryOperator) -> Self {
        let position = PositionRange::concat(&left.get_position(), &right.get_position());

        Self {
            left,
            right,
            operator,
            position,
            id: new_ast_id!(),
        }
    }
}

impl_ast_node!(BinaryExpr, visit_binary);

use std::fmt::Display;

use serde::Serialize;

use crate::ast::ASTNode;
use crate::resolver::ResolvedType;
use crate::{impl_ast_node, impl_positioned, impl_typed};
use crate::lexer::token::{PositionRange, TokenType};

#[derive(Serialize, Clone, Copy)]
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
pub struct BinaryExpr {
    pub left: Box<dyn ASTNode>,
    pub right: Box<dyn ASTNode>,
    pub operator: BinaryOperator,
    position: PositionRange,
    resolved_type: Option<ResolvedType>,
}

impl BinaryExpr {
    pub fn new(left: Box<dyn ASTNode>, right: Box<dyn ASTNode>, operator: BinaryOperator) -> Self {
        let position = PositionRange::concat(&left.get_position(), &right.get_position());

        Self {
            left,
            right,
            operator,
            position,
            resolved_type: None,
        }
    }
}

impl_positioned!(BinaryExpr);
impl_typed!(BinaryExpr);
impl_ast_node!(BinaryExpr, visit_binary);
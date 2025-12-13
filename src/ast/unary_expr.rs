use serde::Serialize;

use crate::ast::ASTNode;
use crate::{impl_ast_node, impl_positioned};
use crate::lexer::token::{PositionRange, TokenType};

#[derive(Serialize)]
pub enum UnaryOperator {
    Deref,
    AddressOf,
    Not,
    Negative,
    Semicolon,
}

impl TryFrom<TokenType> for UnaryOperator {
    type Error = String;

    fn try_from(token_type: TokenType) -> Result<Self, String> {
        match token_type {
            TokenType::Star => Ok(UnaryOperator::Deref),
            TokenType::Ampersand => Ok(UnaryOperator::AddressOf),
            TokenType::Not => Ok(UnaryOperator::Not),
            TokenType::Minus => Ok(UnaryOperator::Negative),
            TokenType::Semicolon => Ok(UnaryOperator::Semicolon),
            _ => Err(format!("Invalid token for binary operator: {:?}", token_type)),
        }
    }
}

#[derive(Serialize)]
pub struct UnaryExpr {
    pub expr: Box<dyn ASTNode>,
    pub operator: UnaryOperator,
    position: PositionRange,
}

impl UnaryExpr {
    pub fn new(expr: Box<dyn ASTNode>, operator: UnaryOperator, position: PositionRange) -> Self {
        Self {
            expr,
            operator,
            position,
        }
    }
}    

impl_positioned!(UnaryExpr);
impl_ast_node!(UnaryExpr, visit_unary);

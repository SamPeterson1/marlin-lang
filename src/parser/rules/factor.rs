use std::fmt;

use crate::{ast::{ASTNode, ASTWrapper}, parser::{ExprParser, ParseRule, ParserCursor, TokenCursor, rules::unary::UnaryRule}, token::TokenType};

pub struct FactorRule {}

impl fmt::Display for FactorRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Factor")
    }
}

impl ParseRule<Box<dyn ASTNode>> for FactorRule {
    fn check_match(&self, _cursor: ParserCursor) -> bool {
        true
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        let mut expr = parser.apply_rule(UnaryRule {}, "unary expression", None)?;

        while let Some(operator) = parser.try_consume_match(&[TokenType::Slash, TokenType::Star]) {
            let unary = parser.apply_rule(UnaryRule {}, "unary expression", None)?;
            expr = Box::new(ASTWrapper::new_binary(expr, unary, operator.token_type));
        }

        Some(expr)
    }
}
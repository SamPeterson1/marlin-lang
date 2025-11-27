use std::fmt;

use crate::{ast::{ASTNode, ASTWrapper}, parser::{ExprParser, ParseRule, ParserCursor, TokenCursor, rules::comparison::ComparisonRule}, token::TokenType};

pub struct EqualityRule {}

impl fmt::Display for EqualityRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Equality")
    }
}

impl ParseRule<Box<dyn ASTNode>> for EqualityRule {
    fn check_match(&self, _cursor: ParserCursor) -> bool {
        true
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        let mut expr = parser.apply_rule(ComparisonRule {}, "comparison expression", None)?;

        while let Some(operator) = parser.try_consume_match(&[TokenType::Equal, TokenType::NotEqual]) {
            let comparison = parser.apply_rule(ComparisonRule {}, "comparison expression", None)?;
            expr = Box::new(ASTWrapper::new_binary(expr, comparison, operator.token_type));
        }

        Some(expr)
    }
}
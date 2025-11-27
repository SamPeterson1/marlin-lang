use std::fmt;

use crate::{ast::{ASTNode, ASTWrapper},parser::{ExprParser, ParseRule, ParserCursor, TokenCursor, diagnostic::ErrMsg, rules::equality::EqualityRule}, token::TokenType};

pub struct BooleanFactorRule {}

impl fmt::Display for BooleanFactorRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BooleanFactor")
    }
}

impl ParseRule<Box<dyn ASTNode>> for BooleanFactorRule {
    fn check_match(&self, _cursor: ParserCursor) -> bool {
        true
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        let mut equality = parser.apply_rule(EqualityRule {}, "equality expression", Some(ErrMsg::ExpectedExpression));
        let mut expr = equality?;

        while let Some(operator) = parser.try_consume(TokenType::And) {
            equality = parser.apply_rule(EqualityRule {}, "equality expression", Some(ErrMsg::ExpectedExpression));
            expr = Box::new(ASTWrapper::new_binary(expr, equality?, operator.token_type));
        }

        Some(expr)
    }
}


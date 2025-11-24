use crate::{ast::{ASTNode, ASTWrapper}, logger::Log, parser::{ExprParser, ParseRule, ParserCursor, TokenCursor, rules::{call::CallRule, primary::PrimaryRule}}, token::TokenType};
use std::fmt;

pub struct UnaryRule {}

impl fmt::Display for UnaryRule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Unary")
    }
}

impl ParseRule<Box<dyn ASTNode>> for UnaryRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        true
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        if let Some(operator) = parser.try_match(&[TokenType::Not, TokenType::Minus]) {
            let unary = parser.apply_rule(UnaryRule {}, "unary expression", None)?;
            Some(Box::new(ASTWrapper::new_unary(unary, operator)))
        } else {
            parser.apply_rule(PrimaryRule {}, "primary expression", None)
        }
    }
}
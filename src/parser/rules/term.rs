use crate::{ast::{ASTNode, ASTWrapper}, logger::Log, parser::{ExprParser, ParseRule, ParserCursor, TokenCursor, rules::factor::FactorRule}, token::TokenType};
use std::fmt;

pub struct TermRule {}

impl fmt::Display for TermRule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Term")
    }
}

impl ParseRule<Box<dyn ASTNode>> for TermRule {
    fn check_match(&self, _cursor: ParserCursor) -> bool {
        true
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        let mut expr =  parser.apply_rule(FactorRule {}, "factor expression", None)?;

        while let Some(operator) = parser.try_match(&[TokenType::Minus, TokenType::Plus]) {
            let factor =  parser.apply_rule(FactorRule {}, "factor expression", None)?;
            expr = Box::new(ASTWrapper::new_binary(expr, factor, operator.token_type));
        }

        Some(expr)
    }
}
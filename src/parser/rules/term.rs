use crate::{ast::{ASTNode, ASTWrapper}, logger::Log, parser::{ExprParser, ParseRule, rules::factor::FactorRule}, token::TokenType};
use std::fmt;

pub struct TermRule {}

impl fmt::Display for TermRule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Term")
    }
}

//factor (("-" | "+") factor)*
impl ParseRule<Box<dyn ASTNode>> for TermRule {
    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        parser.log_debug(&format!("Entering term parser. Current token {:?}", parser.cur()));

        let mut factor = parser.apply_rule(FactorRule {});
        parser.log_parse_result(&factor, "factor expression");
        let mut expr = factor?;

        while let Some(operator) = parser.try_match(&[TokenType::Minus, TokenType::Plus]) {
            factor = parser.apply_rule(FactorRule {});
            parser.log_parse_result(&factor, "factor expression");
            expr = Box::new(ASTWrapper::new_binary(expr, factor?, operator.token_type));
        }

        Some(expr)
    }
}
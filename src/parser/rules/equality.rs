use std::fmt;

use crate::{ast::{ASTNode, ASTWrapper}, logger::Log, parser::{ExprParser, ParseRule, rules::comparison::ComparisonRule}, token::TokenType};

pub struct EqualityRule {}

impl fmt::Display for EqualityRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Equality")
    }
}

//comparison (( "!=" | "==") comparison)*
impl ParseRule<Box<dyn ASTNode>> for EqualityRule {
    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        parser.log_debug(&format!("Entering equality parser. Current token {:?}", parser.cur()));

        let mut comparison = parser.apply_rule(ComparisonRule {});
        parser.log_parse_result(&comparison, "comparison expression");

        let mut expr = comparison?;

        while let Some(operator) = parser.try_match(&[TokenType::Equal, TokenType::NotEqual]) {
            comparison = parser.apply_rule(ComparisonRule {});
            parser.log_parse_result(&comparison, "comparison expression");

            expr = Box::new(ASTWrapper::new_binary(expr, comparison?, operator.token_type));
        }

        Some(expr)
    }
}
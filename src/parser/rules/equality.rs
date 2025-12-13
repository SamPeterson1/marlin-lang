use std::fmt;

use crate::ast::{ASTNode, binary_expr::{BinaryExpr, BinaryOperator}};
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::comparison::ComparisonRule;
use crate::lexer::token::TokenType;

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
            let binary_operator: BinaryOperator = operator.value.try_into().unwrap();

            let comparison = parser.apply_rule(ComparisonRule {}, "comparison expression", None)?;
            expr = Box::new(BinaryExpr::new(expr, comparison, binary_operator));
        }

        Some(expr)
    }
}
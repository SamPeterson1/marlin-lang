use std::fmt;

use crate::ast::{ASTNode, binary_expr::{BinaryExpr, BinaryOperator}};
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::factor::FactorRule;
use crate::lexer::token::TokenType;

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

        while let Some(operator) = parser.try_consume_match(&[TokenType::Minus, TokenType::Plus]) {
            let binary_operator: BinaryOperator = operator.value.try_into().unwrap();

            let factor =  parser.apply_rule(FactorRule {}, "factor expression", None)?;
            expr = Box::new(BinaryExpr::new(expr, factor, binary_operator));
        }

        Some(expr)
    }
}
use std::fmt;

use crate::ast::{ASTNode, binary_expr::{BinaryExpr, BinaryOperator}};
use crate::diagnostic::ErrMsg;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::equality::EqualityRule;
use crate::lexer::token::TokenType;

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
            let binary_operator: BinaryOperator = operator.value.try_into().unwrap();

            equality = parser.apply_rule(EqualityRule {}, "equality expression", Some(ErrMsg::ExpectedExpression));
            expr = Box::new(BinaryExpr::new(expr, equality?, binary_operator));
        }

        Some(expr)
    }
}


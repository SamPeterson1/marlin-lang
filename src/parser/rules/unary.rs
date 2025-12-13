use std::fmt;

use crate::ast::{ASTNode, unary_expr::{UnaryExpr, UnaryOperator}};
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::member_access::MemberAccessRule;
use crate::lexer::token::TokenType;

pub struct UnaryRule {}

impl fmt::Display for UnaryRule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Unary")
    }
}

impl ParseRule<Box<dyn ASTNode>> for UnaryRule {
    fn check_match(&self,  _cursor: ParserCursor) -> bool {
        true
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        parser.begin_range();

        if let Some(operator) = parser.try_consume_match(&[TokenType::Not, TokenType::Minus, TokenType::Star, TokenType::Ampersand]) {
            let unary_operator: UnaryOperator = operator.value.try_into().unwrap();
            let unary = parser.apply_rule(UnaryRule {}, "unary expression", None)?;
            Some(Box::new(UnaryExpr::new(unary, unary_operator, parser.end_range())))
        } else {
            parser.apply_rule(MemberAccessRule {}, "member access expression", None)
        }
    }
}
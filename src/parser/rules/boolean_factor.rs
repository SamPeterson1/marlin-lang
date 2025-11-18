use std::fmt::{self, Binary};

use crate::{expr::{ASTNode, ASTWrapper, binary_expr::BinaryExpr}, logger::Log, parser::{ExprParser, ParseRule, rules::equality::EqualityRule}, token::TokenType};

pub struct BooleanFactorRule {}

impl fmt::Display for BooleanFactorRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BooleanFactor")
    }
}

//equality (or equality)*
impl ParseRule<Box<dyn ASTNode>> for BooleanFactorRule {
    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        parser.log_debug(&format!("Entering boolean factor parser. Current token {:?}", parser.cur()));

        let mut equality = parser.apply_rule(EqualityRule {});
        parser.log_parse_result(&equality, "equality expression");
        let mut expr = equality?;

        while let Some(operator) = parser.try_consume(TokenType::And) {
            equality = parser.apply_rule(EqualityRule {});
            parser.log_parse_result(&equality, "equality expression");

            expr = Box::new(ASTWrapper::new_binary(expr, equality?, operator.token_type));
        }

        Some(expr)
    }
}


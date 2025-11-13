use std::fmt::{self, Binary};

use crate::{expr::{Expr, assignment_expr::AssignmentExpr, binary_expr::BinaryExpr, block_expr::BlockExpr, static_array_expr::StaticArrayExpr, var_expr::VarExpr}, logger::{Log, Logger}, parser::{ExprParser, ParseRule, rules::{equality::EqualityRule, statement::StatementRule, var::VarRule}}, token::{Position, PositionRange, TokenType}};

pub struct BooleanFactorRule {}

impl fmt::Display for BooleanFactorRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BooleanFactor")
    }
}

//equality (or equality)*
impl ParseRule<Box<dyn Expr>> for BooleanFactorRule {
    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn Expr>> {
        parser.log_debug(&format!("Entering boolean factor parser. Current token {:?}", parser.cur()));

        let mut equality = parser.apply_rule(EqualityRule {});
        parser.log_parse_result(&equality, "equality expression");
        let mut expr = equality?;

        while let Some(operator) = parser.try_consume(TokenType::And) {
            equality = parser.apply_rule(EqualityRule {});
            parser.log_parse_result(&equality, "equality expression");

            expr = Box::new(BinaryExpr::new(expr, equality?, operator.token_type));
        }

        Some(expr)
    }
}


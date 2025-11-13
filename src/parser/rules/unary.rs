use crate::{expr::{Expr, binary_expr::BinaryExpr, unary_expr::UnaryExpr}, item::StructItem, logger::Log, parser::{ExprParser, ParseRule, rules::call::CallRule}, token::{Position, PositionRange, TokenType}};
use std::fmt;
use std::collections::HashMap;

pub struct UnaryRule {}

impl fmt::Display for UnaryRule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Unary")
    }
}

//(("!" | "-") unary) | call
impl ParseRule<Box<dyn Expr>> for UnaryRule {
    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn Expr>> {
        parser.log_debug(&format!("Entering unary parser. Current token {:?}", parser.cur()));

        if let Some(operator) = parser.try_match(&[TokenType::Not, TokenType::Minus]) {
            let unary = parser.apply_rule(UnaryRule {});
            parser.log_parse_result(&unary, "unary expression");
            Some(Box::new(UnaryExpr::new(unary?, operator)))
        } else {
            parser.apply_rule(CallRule {})
        }
    }
}

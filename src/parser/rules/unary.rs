use crate::{expr::{ASTNode, ASTWrapper, unary_expr::UnaryExpr}, logger::Log, parser::{ExprParser, ParseRule, rules::call::CallRule}, token::{Position, PositionRange, TokenType}};
use std::fmt;

pub struct UnaryRule {}

impl fmt::Display for UnaryRule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Unary")
    }
}

//(("!" | "-") unary) | call
impl ParseRule<Box<dyn ASTNode>> for UnaryRule {
    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        parser.log_debug(&format!("Entering unary parser. Current token {:?}", parser.cur()));

        if let Some(operator) = parser.try_match(&[TokenType::Not, TokenType::Minus]) {
            let unary = parser.apply_rule(UnaryRule {});
            parser.log_parse_result(&unary, "unary expression");
            Some(Box::new(ASTWrapper::new_unary(unary?, operator)))
        } else {
            parser.apply_rule(CallRule {})
        }
    }
}
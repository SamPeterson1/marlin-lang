use crate::{expr::ASTNode, parser::{ExprParser, ParseRule, diagnostic}, token::{Position, PositionRange, TokenType}};
use std::fmt;

use super::{function_item::FunctionRule, struct_item::StructRule};

pub struct ItemRule {}

impl fmt::Display for ItemRule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Item")
    }
}

impl ParseRule<Box<dyn ASTNode>> for ItemRule {
    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        let cur = parser.cur();

        match cur.token_type {
            TokenType::Struct => parser.apply_rule_boxed(StructRule {}),
            TokenType::Fn => parser.apply_rule_boxed(FunctionRule {}),
            _ => {
                parser.push_diagnostic(diagnostic::err_expected_item(PositionRange::new(Position::new(0, 0))));
                parser.advance();

                None
            }
        }
    }
}
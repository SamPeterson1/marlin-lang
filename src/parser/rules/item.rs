use crate::{ast::ASTNode, parser::{ExprParser, ParseRule, ParserCursor, diagnostic, rules::main_item::MainItemRule}, token::{Position, PositionRange, TokenType}};
use std::fmt;

use super::{function_item::FunctionRule, struct_item::StructRule};

pub struct ItemRule {}

impl fmt::Display for ItemRule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Item")
    }
}

impl ParseRule<Box<dyn ASTNode>> for ItemRule {
    fn check_match(&self, _cursor: ParserCursor) -> bool {
        true
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        if (MainItemRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(MainItemRule {}, "main item", None);
        }

        if (FunctionRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(FunctionRule {}, "function item", None);
        }

        if ((StructRule {})).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(StructRule {}, "struct item", None);
        }

        None
    }
}
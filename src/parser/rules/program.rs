use std::fmt;

use crate::ast::program::Program;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::item::ItemRule;

pub struct ProgramRule {}

impl fmt::Display for ProgramRule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Program")
    }
}

impl ParseRule<Program> for ProgramRule {
    fn check_match(&self, _cursor: ParserCursor) -> bool {
        true
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<Program> {
        parser.begin_range();
        let mut items = Vec::new();

        while !parser.is_at_end() {
            if let Some(item) = parser.apply_rule(ItemRule {}, "item", None) {
                items.push(item);
            }

            if !((ItemRule {})).check_match(parser.get_cursor()) {
                // Skip invalid tokens to avoid infinite loops
                parser.next();
            }
        }

        Some(Program::new(items, parser.end_range()))
    }
}
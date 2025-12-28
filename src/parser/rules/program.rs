use std::fmt;

use crate::ast::Program;
use crate::parser::rules::require::RequireRule;
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
        let mut requires = Vec::new();

        while let Some(require) = parser.apply_rule(RequireRule {}, "require", None) {
            requires.extend(require);
        }

        let mut items = Vec::new();

        while let Some(item) = parser.apply_rule(ItemRule {}, "item", None) {
            items.push(item);
        }

        Some(Program::new(requires, items, parser.end_range()))
    }
}
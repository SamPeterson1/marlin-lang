use chrono::format::Parsed;

use crate::{ast::{ASTWrapper, program::Program, struct_item::StructItem}, logger::Log, parser::{ExprParser, ParseRule, ParserCursor, TokenCursor, diagnostic, rules::{constructor_item::ConstructorRule, item::ItemRule, parsed_type::ParsedTypeRule}}, token::{Position, PositionRange, TokenType}};
use std::fmt;
use std::collections::HashMap;

pub struct ProgramRule {}

impl fmt::Display for ProgramRule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Program")
    }
}

impl ParseRule<ASTWrapper<Program>> for ProgramRule {
    fn check_match(&self, _cursor: ParserCursor) -> bool {
        true
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<ASTWrapper<Program>> {
        let mut items = Vec::new();

        while !parser.is_at_end() {
            if let Some(item) = parser.apply_rule(ItemRule {}, "item", None) {
                items.push(item);
            }
        }

        Some(ASTWrapper::new_program(items))
    }
}
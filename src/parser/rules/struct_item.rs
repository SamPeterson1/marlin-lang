use chrono::format::Parsed;

use crate::{ast::{ASTWrapper, struct_item::StructItem}, logger::Log, parser::{ExprParser, ParseRule, ParserCursor, TokenCursor, diagnostic, rules::{constructor_item::ConstructorRule, parsed_type::ParsedTypeRule}}, token::{Position, PositionRange, Token, TokenType}};
use std::fmt;
use std::collections::HashMap;

pub struct StructRule {}

impl fmt::Display for StructRule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Struct")
    }
}

impl ParseRule<ASTWrapper<StructItem>> for StructRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::Struct).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<ASTWrapper<StructItem>> {
        let struct_token = parser.try_consume(TokenType::Struct)?;

        let struct_identifier = parser.consume_or_diagnostic(TokenType::Identifier)?.get_string().to_string();

        parser.consume_or_diagnostic(TokenType::LeftCurly);

        let mut members = Vec::new();
        while let Some(member_type) = parser.apply_rule(ParsedTypeRule {}, "struct member type", None) {
            let member_identifier = parser.consume_or_diagnostic(TokenType::Identifier)?.get_string().to_string();
            parser.consume_or_diagnostic(TokenType::Semicolon);

            members.push((member_type, member_identifier));
        }
        
        let mut constructors = Vec::new();

        while let Some(constructor) = parser.apply_rule(ConstructorRule {}, "struct constructor", None) {
            constructors.push(constructor);
        }

        parser.consume_or_diagnostic(TokenType::RightCurly);

        let position = PositionRange::concat(&struct_token.position, &parser.cur().position);

        Some(ASTWrapper::new_struct_item(struct_identifier, members, constructors, position))
    }
}
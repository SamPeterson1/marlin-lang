use std::fmt;

use crate::ast::struct_item::StructItem;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::{constructor_item::ConstructorRule, parsed_type::ParsedTypeRule};
use crate::lexer::token::TokenType;

pub struct StructRule {}

impl fmt::Display for StructRule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Struct")
    }
}

impl ParseRule<StructItem> for StructRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::Struct).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<StructItem> {
        parser.begin_range();
        parser.try_consume(TokenType::Struct)?;

        let struct_identifier = parser.consume_or_diagnostic(TokenType::AnyIdentifier)?.unwrap_identifier();

        parser.consume_or_diagnostic(TokenType::LeftCurly);

        let mut members = Vec::new();
        while let Some(member_type) = parser.apply_rule(ParsedTypeRule {}, "struct member type", None) {
            let member_identifier = parser.consume_or_diagnostic(TokenType::AnyIdentifier)?.unwrap_identifier();
            parser.consume_or_diagnostic(TokenType::Semicolon);

            members.push((member_type, member_identifier));
        }
        
        let mut constructors = Vec::new();

        while let Some(constructor) = parser.apply_rule(ConstructorRule {}, "struct constructor", None) {
            constructors.push(constructor);
        }

        parser.consume_or_diagnostic(TokenType::RightCurly);

        Some(StructItem::new(struct_identifier, members, constructors, parser.end_range()))
    }
}
use std::fmt;

use crate::ast::StructItem;
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

            if let Some(token) = parser.consume_or_diagnostic(TokenType::AnyIdentifier) {
                let member_identifier = token.unwrap_identifier();
                parser.consume_or_diagnostic(TokenType::Semicolon);

                members.push((member_type, member_identifier));
            }
        }
        
        let mut constructors = Vec::new();

        while let Some(constructor) = parser.apply_rule(ConstructorRule {}, "struct constructor", None) {
            constructors.push(constructor);
        }

        parser.consume_or_diagnostic(TokenType::RightCurly);

        Some(StructItem::new(struct_identifier, members, constructors, parser.end_range()))
    }
}

use crate::logger::CONSOLE_LOGGER;
#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::{Token, TokenType, PositionRange};

    fn create_token(token_type: TokenType) -> Token {
        Token::new(token_type, PositionRange::zero())
    }

    #[test]
    fn test_struct_rule_check_match_with_struct() {
        let rule = StructRule {};
        let tokens = vec![
            create_token(TokenType::Struct),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_struct_rule_check_match_without_struct() {
        let rule = StructRule {};
        let tokens = vec![
            create_token(TokenType::Fn),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(!rule.check_match(cursor));
    }

    #[test]
    fn test_parse_empty_struct() {
        let rule = StructRule {};
        let tokens = vec![
            create_token(TokenType::Struct),
            create_token(TokenType::Identifier("Person".to_string())),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let struct_item = result.unwrap();
        assert_eq!(struct_item.name.data, "Person");
        assert_eq!(struct_item.members.len(), 0);
        assert_eq!(struct_item.constructors.len(), 0);
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid empty struct");
    }

    #[test]
    fn test_parse_struct_with_single_member() {
        let rule = StructRule {};
        let tokens = vec![
            create_token(TokenType::Struct),
            create_token(TokenType::Identifier("Point".to_string())),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let struct_item = result.unwrap();
        assert_eq!(struct_item.name.data, "Point");
        assert_eq!(struct_item.members.len(), 1);
        assert_eq!(struct_item.members[0].1.data, "x");
        assert_eq!(struct_item.constructors.len(), 0);
        assert!(diagnostics.is_empty(), "Expected no diagnostics for struct with single member");
    }

    #[test]
    fn test_parse_struct_with_multiple_members() {
        let rule = StructRule {};
        let tokens = vec![
            create_token(TokenType::Struct),
            create_token(TokenType::Identifier("Rectangle".to_string())),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("width".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Double),
            create_token(TokenType::Identifier("height".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Bool),
            create_token(TokenType::Identifier("visible".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let struct_item = result.unwrap();
        assert_eq!(struct_item.name.data, "Rectangle");
        assert_eq!(struct_item.members.len(), 3);
        assert_eq!(struct_item.members[0].1.data, "width");
        assert_eq!(struct_item.members[1].1.data, "height");
        assert_eq!(struct_item.members[2].1.data, "visible");
        assert_eq!(struct_item.constructors.len(), 0);
        assert!(diagnostics.is_empty(), "Expected no diagnostics for struct with multiple members");
    }

    #[test]
    fn test_parse_struct_with_constructor() {
        let rule = StructRule {};
        let tokens = vec![
            create_token(TokenType::Struct),
            create_token(TokenType::Identifier("Person".to_string())),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Identifier("String".to_string())),
            create_token(TokenType::Identifier("name".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::DollarSign),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Identifier("String".to_string())),
            create_token(TokenType::Identifier("n".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let struct_item = result.unwrap();
        assert_eq!(struct_item.name.data, "Person");
        assert_eq!(struct_item.members.len(), 1);
        assert_eq!(struct_item.constructors.len(), 1);
        assert!(diagnostics.is_empty(), "Expected no diagnostics for struct with constructor");
    }

    #[test]
    fn test_parse_struct_with_multiple_constructors() {
        let rule = StructRule {};
        let tokens = vec![
            create_token(TokenType::Struct),
            create_token(TokenType::Identifier("Point".to_string())),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("y".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::DollarSign),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::DollarSign),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Comma),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("y".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let struct_item = result.unwrap();
        assert_eq!(struct_item.name.data, "Point");
        assert_eq!(struct_item.members.len(), 2);
        assert_eq!(struct_item.constructors.len(), 2);
        assert!(diagnostics.is_empty(), "Expected no diagnostics for struct with multiple constructors");
    }

    #[test]
    fn test_parse_struct_with_custom_types() {
        let rule = StructRule {};
        let tokens = vec![
            create_token(TokenType::Struct),
            create_token(TokenType::Identifier("Vehicle".to_string())),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Identifier("Engine".to_string())),
            create_token(TokenType::Identifier("engine".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("Tire".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::RightSquare),
            create_token(TokenType::Identifier("tires".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let struct_item = result.unwrap();
        assert_eq!(struct_item.name.data, "Vehicle");
        assert_eq!(struct_item.members.len(), 2);
        assert_eq!(struct_item.members[0].1.data, "engine");
        assert_eq!(struct_item.members[1].1.data, "tires");
        assert!(diagnostics.is_empty(), "Expected no diagnostics for struct with custom types");
    }

    #[test]
    fn test_parse_missing_struct_keyword() {
        let rule = StructRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("Person".to_string())),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_missing_struct_name() {
        let rule = StructRule {};
        let tokens = vec![
            create_token(TokenType::Struct),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(!result.is_some());
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing struct name");
        assert!(diagnostics.iter().any(|d| d.message.contains("identifier")));
    }

    #[test]
    fn test_parse_missing_left_brace() {
        let rule = StructRule {};
        let tokens = vec![
            create_token(TokenType::Struct),
            create_token(TokenType::Identifier("Person".to_string())),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("age".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing left brace");
        assert!(diagnostics.iter().any(|d| d.message.contains("'{'")));
    }

    #[test]
    fn test_parse_missing_right_brace() {
        let rule = StructRule {};
        let tokens = vec![
            create_token(TokenType::Struct),
            create_token(TokenType::Identifier("Person".to_string())),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("age".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing right brace");
        assert!(diagnostics.iter().any(|d| d.message.contains("'}'")));
    }

    #[test]
    fn test_parse_missing_member_semicolon() {
        let rule = StructRule {};
        let tokens = vec![
            create_token(TokenType::Struct),
            create_token(TokenType::Identifier("Person".to_string())),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("age".to_string())),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing semicolon");
        assert!(diagnostics.iter().any(|d| d.message.contains("';'")));
    }

    #[test]
    fn test_parse_missing_member_name() {
        let rule = StructRule {};
        let tokens = vec![
            create_token(TokenType::Struct),
            create_token(TokenType::Identifier("Person".to_string())),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Int),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing member name");
        assert!(diagnostics.iter().any(|d| d.message.contains("identifier")));
    }

    #[test]
    fn test_parse_complex_struct() {
        let rule = StructRule {};
        let tokens = vec![
            create_token(TokenType::Struct),
            create_token(TokenType::Identifier("GameState".to_string())),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Identifier("Player".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::RightSquare),
            create_token(TokenType::Identifier("players".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Int),
            create_token(TokenType::Star),
            create_token(TokenType::Identifier("score".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Bool),
            create_token(TokenType::Ampersand),
            create_token(TokenType::Identifier("isRunning".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::DollarSign),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let struct_item = result.unwrap();
        assert_eq!(struct_item.name.data, "GameState");
        assert_eq!(struct_item.members.len(), 3);
        assert_eq!(struct_item.constructors.len(), 1);
        assert!(diagnostics.is_empty(), "Expected no diagnostics for complex struct");
    }
}
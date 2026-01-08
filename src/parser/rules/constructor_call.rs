use std::fmt;

use crate::ast::ConstructorCallExpr;
use crate::diagnostic::ErrMsg;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::arguments::ArgumentsRule;
use crate::lexer::token::TokenType;

pub struct ConstructorCallRule {}

impl fmt::Display for ConstructorCallRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ConstructorCall")
    }
}

impl ParseRule<ConstructorCallExpr> for ConstructorCallRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::New);
        cursor.try_consume(TokenType::DollarSign).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<ConstructorCallExpr> {
        parser.begin_range();

        let is_heap = parser.try_consume(TokenType::New).is_some();
        parser.consume_or_diagnostic(TokenType::DollarSign);

        let type_name = parser.consume_or_diagnostic(TokenType::AnyIdentifier)?.unwrap_identifier();

        let arguments = parser.apply_rule(ArgumentsRule {}, "constructor arguments", Some(ErrMsg::ExpectedArguments))?;
        
        Some(ConstructorCallExpr::new(type_name, arguments, is_heap, parser.end_range()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::{Token, TokenType, PositionRange};
    use crate::logger::CONSOLE_LOGGER;

    fn create_token(token_type: TokenType) -> Token {
        Token::new(token_type, PositionRange::zero())
    }

    #[test]
    fn test_constructor_call_rule_check_match_with_new_and_dollar() {
        let rule = ConstructorCallRule {};
        let tokens = vec![
            create_token(TokenType::New),
            create_token(TokenType::DollarSign),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_constructor_call_rule_check_match_with_dollar_only() {
        let rule = ConstructorCallRule {};
        let tokens = vec![
            create_token(TokenType::DollarSign),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_constructor_call_rule_check_match_without_dollar() {
        let rule = ConstructorCallRule {};
        let tokens = vec![
            create_token(TokenType::New),
            create_token(TokenType::Identifier("Test".to_string())),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(!rule.check_match(cursor));
    }

    #[test]
    fn test_parse_heap_constructor_call() {
        let rule = ConstructorCallRule {};
        let tokens = vec![
            create_token(TokenType::New),
            create_token(TokenType::DollarSign),
            create_token(TokenType::Identifier("Person".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::StringLiteral("John".to_string())),
            create_token(TokenType::Comma),
            create_token(TokenType::IntLiteral(30)),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let constructor_call = result.unwrap();
        assert!(constructor_call.is_heap);
        assert_eq!(constructor_call.type_name.as_ref(), "Person");
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid heap constructor call");
    }

    #[test]
    fn test_parse_stack_constructor_call() {
        let rule = ConstructorCallRule {};
        let tokens = vec![
            create_token(TokenType::DollarSign),
            create_token(TokenType::Identifier("Point".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Comma),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let constructor_call = result.unwrap();
        assert!(!constructor_call.is_heap);
        assert_eq!(constructor_call.type_name.as_ref(), "Point");
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid stack constructor call");
    }

    #[test]
    fn test_parse_constructor_call_empty_arguments() {
        let rule = ConstructorCallRule {};
        let tokens = vec![
            create_token(TokenType::DollarSign),
            create_token(TokenType::Identifier("Empty".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let constructor_call = result.unwrap();
        assert_eq!(constructor_call.arguments.len(), 0);
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid empty constructor call");
    }

    #[test]
    fn test_parse_missing_dollar_sign() {
        let rule = ConstructorCallRule {};
        let tokens = vec![
            create_token(TokenType::New),
            create_token(TokenType::Identifier("Test".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should not fail because missing dollar sign is recoverable
        assert!(!result.is_none());
        // Should have diagnostic for missing dollar sign
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing dollar sign");
        assert!(diagnostics.iter().any(|d| d.message.contains("'$'")));
    }

    #[test]
    fn test_parse_missing_type_name() {
        let rule = ConstructorCallRule {};
        let tokens = vec![
            create_token(TokenType::DollarSign),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because identifier is required after dollar sign
        assert!(result.is_none());
        // Should have diagnostic for expected identifier
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing identifier");
        assert!(diagnostics.iter().any(|d| d.message.contains("identifier")));
    }

    #[test]
    fn test_parse_invalid_identifier() {
        let rule = ConstructorCallRule {};
        let tokens = vec![
            create_token(TokenType::DollarSign),
            create_token(TokenType::IntLiteral(123)), // Invalid - must be identifier
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because identifier is required, not integer literal
        assert!(result.is_none());
        // Should have diagnostic for expected identifier
        assert!(!diagnostics.is_empty(), "Expected diagnostic for expected identifier");
        assert!(diagnostics.iter().any(|d| d.message.contains("identifier")));
    }

    #[test]
    fn test_parse_missing_arguments() {
        let rule = ConstructorCallRule {};
        let tokens = vec![
            create_token(TokenType::DollarSign),
            create_token(TokenType::Identifier("Test".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because arguments are required
        assert!(result.is_none());
        // Should have diagnostic for expected arguments
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing arguments");
        assert!(diagnostics.iter().any(|d| d.message.contains("expected arguments")));
    }

    #[test]
    fn test_parse_only_dollar_sign() {
        let rule = ConstructorCallRule {};
        let tokens = vec![
            create_token(TokenType::DollarSign),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because identifier is required after dollar sign
        assert!(result.is_none());
        // Should have diagnostic for missing identifier
        assert!(!diagnostics.is_empty(), "Expected diagnostic for incomplete constructor call");
        assert!(diagnostics.iter().any(|d| d.message.contains("identifier")));
    }

    #[test]
    fn test_parse_new_without_dollar() {
        let rule = ConstructorCallRule {};
        let tokens = vec![
            create_token(TokenType::New),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because dollar sign is required after 'new'
        assert!(result.is_none());
        // Should have diagnostic for missing dollar sign
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing dollar sign after 'new'");
        assert!(diagnostics.iter().any(|d| d.message.contains("'$'")));
    }

    #[test]
    fn test_parse_invalid_type_name() {
        let rule = ConstructorCallRule {};
        let tokens = vec![
            create_token(TokenType::DollarSign),
            create_token(TokenType::IntLiteral(123)), // Invalid type name
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because type name must be identifier
        assert!(result.is_none());
        // Should have diagnostic for expected identifier
        assert!(!diagnostics.is_empty(), "Expected diagnostic for invalid type name");
        assert!(diagnostics.iter().any(|d| d.message.contains("identifier")));
    }

    #[test]
    fn test_parse_malformed_arguments() {
        let rule = ConstructorCallRule {};
        let tokens = vec![
            create_token(TokenType::DollarSign),
            create_token(TokenType::Identifier("Test".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(42)),
            // Missing closing paren - parser can recover from this
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should succeed because parser can recover from missing closing paren
        assert!(result.is_some());
        let constructor_call = result.unwrap();
        assert_eq!(constructor_call.type_name.as_ref(), "Test");
        assert_eq!(constructor_call.arguments.len(), 1);
        // Should have diagnostic for missing closing paren, but not fatal
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing closing paren");
        assert!(diagnostics.iter().any(|d| d.message.contains("')'")));
    }

    #[test]
    fn test_parse_complex_constructor_with_nested_calls() {
        let rule = ConstructorCallRule {};
        let tokens = vec![
            create_token(TokenType::New),
            create_token(TokenType::DollarSign),
            create_token(TokenType::Identifier("Container".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::DollarSign),
            create_token(TokenType::Identifier("Item".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::RightParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let constructor_call = result.unwrap();
        assert!(constructor_call.is_heap);
        assert_eq!(constructor_call.type_name.as_ref(), "Container");
        assert_eq!(constructor_call.arguments.len(), 1);
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid nested constructor calls");
    }
}
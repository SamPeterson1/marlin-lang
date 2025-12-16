use std::fmt;

use crate::ast::Parameters;
use crate::diagnostic::ErrMsg;
use crate::logger::Log;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::parsed_type::ParsedTypeRule;
use crate::lexer::token::TokenType;

pub struct ParametersRule {}

impl fmt::Display for ParametersRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parameters")
    }
}

impl ParseRule<Parameters> for ParametersRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::LeftParen).is_some()
    }
    
    fn parse(&self, parser: &mut ExprParser) -> Option<Parameters> {
        parser.begin_range();
        parser.try_consume(TokenType::LeftParen)?;

        let mut parameters = Vec::new();
        
        if let Some(parsed_type) = parser.apply_rule(ParsedTypeRule {}, "first parameter type", None) {
            let identifier = parser.try_consume(TokenType::AnyIdentifier)?;

            parameters.push((parsed_type, identifier.unwrap_identifier()));
        }

        parser.log_debug(&format!("Current token after first parameter parse: {:?}", parser.cur()));

        while let Some(_) = parser.try_consume(TokenType::Comma) {
            let parsed_type = parser.apply_rule(ParsedTypeRule {}, "parameter type", Some(ErrMsg::ExpectedType))?;
            let identifier = parser.consume_or_diagnostic(TokenType::AnyIdentifier)?;

            parameters.push((parsed_type, identifier.unwrap_identifier()));
        }

        parser.consume_or_diagnostic(TokenType::RightParen);

        Some(Parameters::new(parameters, parser.end_range()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::{Token, TokenType, PositionRange};

    fn create_token(token_type: TokenType) -> Token {
        Token::new(token_type, PositionRange::zero())
    }

    #[test]
    fn test_parameters_rule_check_match_with_left_paren() {
        let rule = ParametersRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_parameters_rule_check_match_without_left_paren() {
        let rule = ParametersRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("test".to_string())),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(!rule.check_match(cursor));
    }

    #[test]
    fn test_parse_empty_parameters() {
        let rule = ParametersRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parameters = result.unwrap();
        assert_eq!(parameters.parameters.len(), 0);
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid empty parameters");
    }

    #[test]
    fn test_parse_single_parameter() {
        let rule = ParametersRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parameters = result.unwrap();
        assert_eq!(parameters.parameters.len(), 1);
        assert_eq!(parameters.parameters[0].1.data, "x");
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid single parameter");
    }

    #[test]
    fn test_parse_multiple_parameters() {
        let rule = ParametersRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Comma),
            create_token(TokenType::Identifier("string".to_string())),
            create_token(TokenType::Identifier("name".to_string())),
            create_token(TokenType::Comma),
            create_token(TokenType::Bool),
            create_token(TokenType::Identifier("flag".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parameters = result.unwrap();
        assert_eq!(parameters.parameters.len(), 3);
        assert_eq!(parameters.parameters[0].1.data, "x");
        assert_eq!(parameters.parameters[1].1.data, "name");
        assert_eq!(parameters.parameters[2].1.data, "flag");
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid multiple parameters");
    }

    #[test]
    fn test_parse_missing_left_paren() {
        let rule = ParametersRule {};
        let tokens = vec![
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_none());
        // No diagnostics expected here since the rule simply returns None when there's no left paren
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_parse_missing_right_paren() {
        let rule = ParametersRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should still return Some but with diagnostics
        assert!(result.is_some());
        let parameters = result.unwrap();
        assert_eq!(parameters.parameters.len(), 1);
        
        // Should have diagnostic for missing right paren
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing right paren");
        assert!(diagnostics.iter().any(|d| d.message.contains("')'")));
    }

    #[test]
    fn test_parse_missing_parameter_name() {
        let rule = ParametersRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::Int),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because parameter name is required after type
        assert!(result.is_none());
        // No diagnostics expected since try_consume returns None
    }

    #[test]
    fn test_parse_missing_parameter_name_after_comma() {
        let rule = ParametersRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Comma),
            create_token(TokenType::Identifier("y".to_string())), // This will be parsed as a type name
            create_token(TokenType::RightParen), // Missing identifier after the "y" type
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because identifier is required after the "y" type
        assert!(result.is_none());
        // Should have diagnostic for expected identifier
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing parameter name after type");
        assert!(diagnostics.iter().any(|d| d.message.contains("identifier")));
    }

    #[test]
    fn test_parse_trailing_comma() {
        let rule = ParametersRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Comma),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because there's no parameter after comma
        assert!(result.is_none());
        // Should have diagnostic for expected type after comma
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing parameter after comma");
        assert!(diagnostics.iter().any(|d| d.message.contains("expected type")));
    }

    #[test]
    fn test_parse_invalid_parameter_type() {
        let rule = ParametersRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(123)), // Invalid type
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should return empty parameters since first type parsing fails
        assert!(result.is_some());
        let parameters = result.unwrap();
        assert_eq!(parameters.parameters.len(), 0);
        // May have diagnostics from failed type parsing
    }

    #[test]
    fn test_parse_invalid_parameter_name() {
        let rule = ParametersRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::Int),
            create_token(TokenType::IntLiteral(123)), // Invalid identifier
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because identifier is invalid
        assert!(result.is_none());
        // No diagnostics expected since try_consume returns None for invalid token type
    }

    #[test]
    fn test_parse_double_comma() {
        let rule = ParametersRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Comma),
            create_token(TokenType::Comma), // Double comma - invalid
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("y".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail due to invalid comma sequence
        assert!(result.is_none());
        // Should have diagnostic for expected type after first comma
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing parameter after comma");
        assert!(diagnostics.iter().any(|d| d.message.contains("expected type")));
    }

    #[test]
    fn test_parse_simple_parameter_types() {
        let rule = ParametersRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::Identifier("Vec".to_string())),
            create_token(TokenType::Identifier("numbers".to_string())),
            create_token(TokenType::Comma),
            create_token(TokenType::Identifier("HashMap".to_string())),
            create_token(TokenType::Identifier("map".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parameters = result.unwrap();
        assert_eq!(parameters.parameters.len(), 2);
        assert_eq!(parameters.parameters[0].1.data, "numbers");
        assert_eq!(parameters.parameters[1].1.data, "map");
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid parameter types");
    }
}
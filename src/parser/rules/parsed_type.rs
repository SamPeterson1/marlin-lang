use std::fmt;

use crate::ast::{ArrayModifier, ParsedType};
use crate::diagnostic::ErrMsg;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::parsed_unit_type::ParsedUnitTypeRule;
use crate::lexer::token::TokenType;

pub struct ParsedTypeRule {}

impl fmt::Display for ParsedTypeRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParsedType")
    }
}

impl ParseRule<ParsedType> for ParsedTypeRule {
    fn check_match(&self, cursor: ParserCursor) -> bool {
        (ParsedUnitTypeRule {}).check_match(cursor)
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<ParsedType> {
        parser.begin_range();
        let unit_type = parser.apply_rule(ParsedUnitTypeRule {}, "unit type", Some(ErrMsg::ExpectedType))?;

        let mut array_modifiers = Vec::new();

        while parser.try_consume(TokenType::LeftSquare).is_some() {
            parser.consume_or_diagnostic(TokenType::RightSquare);

            if parser.try_consume(TokenType::Ampersand).is_some() {
                array_modifiers.push(ArrayModifier {is_reference: true});
            } else {
                array_modifiers.push(ArrayModifier {is_reference: false});
            }
        }


        Some(ParsedType::new(unit_type, array_modifiers, parser.end_range()))
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
    fn test_parsed_type_rule_check_match_with_basic_type() {
        let rule = ParsedTypeRule {};
        let tokens = vec![
            create_token(TokenType::Int),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_parsed_type_rule_check_match_with_identifier() {
        let rule = ParsedTypeRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("CustomType".to_string())),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_parsed_type_rule_check_match_with_invalid_token() {
        let rule = ParsedTypeRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(!rule.check_match(cursor));
    }

    #[test]
    fn test_parse_simple_unit_type() {
        let rule = ParsedTypeRule {};
        let tokens = vec![
            create_token(TokenType::Int),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        assert_eq!(parsed_type.array_modifiers.len(), 0);
        assert!(diagnostics.is_empty(), "Expected no diagnostics for simple unit type");
    }

    #[test]
    fn test_parse_unit_type_with_reference() {
        let rule = ParsedTypeRule {};
        let tokens = vec![
            create_token(TokenType::Int),
            create_token(TokenType::Ampersand),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        assert_eq!(parsed_type.array_modifiers.len(), 0);
        assert!(diagnostics.is_empty(), "Expected no diagnostics for unit type with reference");
    }

    #[test]
    fn test_parse_simple_array_type() {
        let rule = ParsedTypeRule {};
        let tokens = vec![
            create_token(TokenType::Int),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::RightSquare),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        assert_eq!(parsed_type.array_modifiers.len(), 1);
        assert!(!parsed_type.array_modifiers[0].is_reference);
        assert!(diagnostics.is_empty(), "Expected no diagnostics for simple array type");
    }

    #[test]
    fn test_parse_array_with_reference() {
        let rule = ParsedTypeRule {};
        let tokens = vec![
            create_token(TokenType::Int),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::RightSquare),
            create_token(TokenType::Ampersand),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        assert_eq!(parsed_type.array_modifiers.len(), 1);
        assert!(parsed_type.array_modifiers[0].is_reference);
        assert!(diagnostics.is_empty(), "Expected no diagnostics for array with reference");
    }

    #[test]
    fn test_parse_reference_unit_type_with_array() {
        let rule = ParsedTypeRule {};
        let tokens = vec![
            create_token(TokenType::Bool),
            create_token(TokenType::Ampersand),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::RightSquare),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        assert_eq!(parsed_type.array_modifiers.len(), 1);
        assert!(!parsed_type.array_modifiers[0].is_reference);
        assert!(diagnostics.is_empty(), "Expected no diagnostics for reference unit type with array");
    }

    #[test]
    fn test_parse_multidimensional_array() {
        let rule = ParsedTypeRule {};
        let tokens = vec![
            create_token(TokenType::Int),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::RightSquare),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::RightSquare),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::RightSquare),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        assert_eq!(parsed_type.array_modifiers.len(), 3);
        assert!(!parsed_type.array_modifiers[0].is_reference);
        assert!(!parsed_type.array_modifiers[1].is_reference);
        assert!(!parsed_type.array_modifiers[2].is_reference);
        assert!(diagnostics.is_empty(), "Expected no diagnostics for multidimensional array");
    }

    #[test]
    fn test_parse_mixed_array_references() {
        let rule = ParsedTypeRule {};
        let tokens = vec![
            create_token(TokenType::Bool),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::RightSquare),
            create_token(TokenType::Ampersand),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::RightSquare),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::RightSquare),
            create_token(TokenType::Ampersand),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        assert_eq!(parsed_type.array_modifiers.len(), 3);
        assert!(parsed_type.array_modifiers[0].is_reference);
        assert!(!parsed_type.array_modifiers[1].is_reference);
        assert!(parsed_type.array_modifiers[2].is_reference);
        assert!(diagnostics.is_empty(), "Expected no diagnostics for mixed array references");
    }

    #[test]
    fn test_parse_custom_type() {
        let rule = ParsedTypeRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("MyStruct".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        assert_eq!(parsed_type.array_modifiers.len(), 0);
        assert!(diagnostics.is_empty(), "Expected no diagnostics for custom type");
    }

    #[test]
    fn test_parse_custom_type_with_reference() {
        let rule = ParsedTypeRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("Person".to_string())),
            create_token(TokenType::Ampersand),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        assert_eq!(parsed_type.array_modifiers.len(), 0);
        assert!(diagnostics.is_empty(), "Expected no diagnostics for custom type with reference");
    }

    #[test]
    fn test_parse_custom_type_array_with_reference() {
        let rule = ParsedTypeRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("Person".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::RightSquare),
            create_token(TokenType::Ampersand),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        assert_eq!(parsed_type.array_modifiers.len(), 1);
        assert!(parsed_type.array_modifiers[0].is_reference);
        assert!(diagnostics.is_empty(), "Expected no diagnostics for custom type array with reference");
    }

    #[test]
    fn test_parse_reference_custom_type_with_array() {
        let rule = ParsedTypeRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("Matrix".to_string())),
            create_token(TokenType::Ampersand),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::RightSquare),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::RightSquare),
            create_token(TokenType::Ampersand),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        assert_eq!(parsed_type.array_modifiers.len(), 2);
        assert!(!parsed_type.array_modifiers[0].is_reference);
        assert!(parsed_type.array_modifiers[1].is_reference);
        assert!(diagnostics.is_empty(), "Expected no diagnostics for reference custom type with arrays");
    }

    #[test]
    fn test_parse_missing_unit_type() {
        let rule = ParsedTypeRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(42)), // Invalid for unit type
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_none());
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing unit type");
        assert!(diagnostics.iter().any(|d| d.message.contains("expected type")));
    }

    #[test]
    fn test_parse_missing_right_square_bracket() {
        let rule = ParsedTypeRule {};
        let tokens = vec![
            create_token(TokenType::Int),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        assert_eq!(parsed_type.array_modifiers.len(), 1);
        assert!(!parsed_type.array_modifiers[0].is_reference);
        
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing right bracket");
        assert!(diagnostics.iter().any(|d| d.message.contains("']'")));
    }

    #[test]
    fn test_parse_multiple_missing_brackets() {
        let rule = ParsedTypeRule {};
        let tokens = vec![
            create_token(TokenType::Double),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        assert_eq!(parsed_type.array_modifiers.len(), 2);
        assert!(!parsed_type.array_modifiers[0].is_reference);
        assert!(!parsed_type.array_modifiers[1].is_reference);
        
        assert!(!diagnostics.is_empty(), "Expected diagnostics for missing right brackets");
        assert_eq!(diagnostics.len(), 2);
        assert!(diagnostics.iter().all(|d| d.message.contains("']'")));
    }

    #[test]
    fn test_parse_complex_type_example() {
        let rule = ParsedTypeRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("HashMap".to_string())),
            create_token(TokenType::Ampersand), // HashMap&
            create_token(TokenType::LeftSquare),
            create_token(TokenType::RightSquare), // []
            create_token(TokenType::LeftSquare),
            create_token(TokenType::RightSquare),
            create_token(TokenType::Ampersand), // []&
            create_token(TokenType::LeftSquare),
            create_token(TokenType::RightSquare), // []
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        // HashMap&[][][&][]
        assert_eq!(parsed_type.array_modifiers.len(), 3);
        assert!(!parsed_type.array_modifiers[0].is_reference); // First []
        assert!(parsed_type.array_modifiers[1].is_reference);  // []&
        assert!(!parsed_type.array_modifiers[2].is_reference); // Last []
        assert!(diagnostics.is_empty(), "Expected no diagnostics for complex type");
    }

    #[test]
    fn test_parse_all_primitive_types() {
        let test_cases = vec![
            TokenType::Int,
            TokenType::Double, 
            TokenType::Bool,
            TokenType::Char,
        ];
        
        for primitive_type in test_cases {
            let rule = ParsedTypeRule {};
            let tokens = vec![
                create_token(primitive_type.clone()),
                create_token(TokenType::EOF),
            ];
            let mut diagnostics = Vec::new();
            let mut parser = ExprParser::new(tokens, &mut diagnostics);
            
            let result = rule.parse(&mut parser);
            
            assert!(result.is_some(), "Failed to parse {:?}", primitive_type);
            let parsed_type = result.unwrap();
            assert!(parsed_type.array_modifiers.is_empty());
            assert!(diagnostics.is_empty(), "Expected no diagnostics for {:?}", primitive_type);
        }
    }

    #[test]
    fn test_parse_array_sequence() {
        let rule = ParsedTypeRule {};
        let tokens = vec![
            create_token(TokenType::Int),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::RightSquare),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::RightSquare),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::RightSquare),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::RightSquare),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        assert_eq!(parsed_type.array_modifiers.len(), 4);
        assert!(parsed_type.array_modifiers.iter().all(|m| !m.is_reference));
        assert!(diagnostics.is_empty(), "Expected no diagnostics for array sequence");
    }
}
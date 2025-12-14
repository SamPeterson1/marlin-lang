use std::fmt;
use std::rc::Rc;

use crate::ast::parsed_type::{ParsedBaseType, ParsedUnitType};
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::lexer::token::{Located, Positioned, TokenType};

pub struct ParsedUnitTypeRule {}

impl fmt::Display for ParsedUnitTypeRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParsedUnitType")
    }
}

impl ParseRule<ParsedUnitType> for ParsedUnitTypeRule {
    fn check_match(&self, cursor: ParserCursor) -> bool {
        cursor.try_match(&[
            TokenType::Int,
            TokenType::Double,
            TokenType::Bool,
            TokenType::Char,
            TokenType::AnyIdentifier,
        ]).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<ParsedUnitType> {
        parser.begin_range();
        let cur= parser.next();

        let base_type = match cur.value {
            TokenType::Int => Located::new(ParsedBaseType::Integer, *cur.get_position()),
            TokenType::Double => Located::new(ParsedBaseType::Double, *cur.get_position()),
            TokenType::Bool => Located::new(ParsedBaseType::Boolean, *cur.get_position()),
            TokenType::Char => Located::new(ParsedBaseType::Char, *cur.get_position()),
            TokenType::Identifier(ref type_name) => {
                Located::new(ParsedBaseType::TypeName(Rc::new(type_name.to_string())), *cur.get_position())
            }
            _ => {
                return None;
            }
        };

        let mut n_pointers = 0;

        while parser.try_consume(TokenType::Star).is_some() {
            n_pointers += 1;
        }

        let is_reference = parser.try_consume(TokenType::Ampersand).is_some();

        Some(ParsedUnitType::new(base_type, n_pointers, is_reference, parser.end_range()))
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
    fn test_parsed_unit_type_rule_check_match_with_int() {
        let rule = ParsedUnitTypeRule {};
        let tokens = vec![
            create_token(TokenType::Int),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_parsed_unit_type_rule_check_match_with_double() {
        let rule = ParsedUnitTypeRule {};
        let tokens = vec![
            create_token(TokenType::Double),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_parsed_unit_type_rule_check_match_with_bool() {
        let rule = ParsedUnitTypeRule {};
        let tokens = vec![
            create_token(TokenType::Bool),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_parsed_unit_type_rule_check_match_with_char() {
        let rule = ParsedUnitTypeRule {};
        let tokens = vec![
            create_token(TokenType::Char),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_parsed_unit_type_rule_check_match_with_identifier() {
        let rule = ParsedUnitTypeRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("MyType".to_string())),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_parsed_unit_type_rule_check_match_with_invalid_token() {
        let rule = ParsedUnitTypeRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(!rule.check_match(cursor));
    }

    #[test]
    fn test_parse_int_type() {
        let rule = ParsedUnitTypeRule {};
        let tokens = vec![
            create_token(TokenType::Int),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let unit_type = result.unwrap();
        assert_eq!(unit_type.n_pointers, 0);
        assert!(!unit_type.is_reference);
        assert!(matches!(unit_type.base_type.data, ParsedBaseType::Integer));
        assert!(diagnostics.is_empty(), "Expected no diagnostics for int type");
    }

    #[test]
    fn test_parse_double_type() {
        let rule = ParsedUnitTypeRule {};
        let tokens = vec![
            create_token(TokenType::Double),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let unit_type = result.unwrap();
        assert_eq!(unit_type.n_pointers, 0);
        assert!(!unit_type.is_reference);
        assert!(matches!(unit_type.base_type.data, ParsedBaseType::Double));
        assert!(diagnostics.is_empty(), "Expected no diagnostics for double type");
    }

    #[test]
    fn test_parse_bool_type() {
        let rule = ParsedUnitTypeRule {};
        let tokens = vec![
            create_token(TokenType::Bool),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let unit_type = result.unwrap();
        assert_eq!(unit_type.n_pointers, 0);
        assert!(!unit_type.is_reference);
        assert!(matches!(unit_type.base_type.data, ParsedBaseType::Boolean));
        assert!(diagnostics.is_empty(), "Expected no diagnostics for bool type");
    }

    #[test]
    fn test_parse_char_type() {
        let rule = ParsedUnitTypeRule {};
        let tokens = vec![
            create_token(TokenType::Char),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let unit_type = result.unwrap();
        assert_eq!(unit_type.n_pointers, 0);
        assert!(!unit_type.is_reference);
        assert!(matches!(unit_type.base_type.data, ParsedBaseType::Char));
        assert!(diagnostics.is_empty(), "Expected no diagnostics for char type");
    }

    #[test]
    fn test_parse_custom_type() {
        let rule = ParsedUnitTypeRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("CustomType".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let unit_type = result.unwrap();
        assert_eq!(unit_type.n_pointers, 0);
        assert!(!unit_type.is_reference);
        if let ParsedBaseType::TypeName(type_name) = &unit_type.base_type.data {
            assert_eq!(type_name.as_str(), "CustomType");
        } else {
            panic!("Expected TypeName variant");
        }
        assert!(diagnostics.is_empty(), "Expected no diagnostics for custom type");
    }

    #[test]
    fn test_parse_pointer_type() {
        let rule = ParsedUnitTypeRule {};
        let tokens = vec![
            create_token(TokenType::Int),
            create_token(TokenType::Star),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let unit_type = result.unwrap();
        assert_eq!(unit_type.n_pointers, 1);
        assert!(!unit_type.is_reference);
        assert!(matches!(unit_type.base_type.data, ParsedBaseType::Integer));
        assert!(diagnostics.is_empty(), "Expected no diagnostics for pointer type");
    }

    #[test]
    fn test_parse_multiple_pointers() {
        let rule = ParsedUnitTypeRule {};
        let tokens = vec![
            create_token(TokenType::Char),
            create_token(TokenType::Star),
            create_token(TokenType::Star),
            create_token(TokenType::Star),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let unit_type = result.unwrap();
        assert_eq!(unit_type.n_pointers, 3);
        assert!(!unit_type.is_reference);
        assert!(matches!(unit_type.base_type.data, ParsedBaseType::Char));
        assert!(diagnostics.is_empty(), "Expected no diagnostics for multiple pointers");
    }

    #[test]
    fn test_parse_reference_type() {
        let rule = ParsedUnitTypeRule {};
        let tokens = vec![
            create_token(TokenType::Double),
            create_token(TokenType::Ampersand),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let unit_type = result.unwrap();
        assert_eq!(unit_type.n_pointers, 0);
        assert!(unit_type.is_reference);
        assert!(matches!(unit_type.base_type.data, ParsedBaseType::Double));
        assert!(diagnostics.is_empty(), "Expected no diagnostics for reference type");
    }

    #[test]
    fn test_parse_pointer_reference() {
        let rule = ParsedUnitTypeRule {};
        let tokens = vec![
            create_token(TokenType::Bool),
            create_token(TokenType::Star),
            create_token(TokenType::Ampersand),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let unit_type = result.unwrap();
        assert_eq!(unit_type.n_pointers, 1);
        assert!(unit_type.is_reference);
        assert!(matches!(unit_type.base_type.data, ParsedBaseType::Boolean));
        assert!(diagnostics.is_empty(), "Expected no diagnostics for pointer reference");
    }

    #[test]
    fn test_parse_multiple_pointers_with_reference() {
        let rule = ParsedUnitTypeRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("MyStruct".to_string())),
            create_token(TokenType::Star),
            create_token(TokenType::Star),
            create_token(TokenType::Star),
            create_token(TokenType::Star),
            create_token(TokenType::Star),
            create_token(TokenType::Ampersand),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let unit_type = result.unwrap();
        assert_eq!(unit_type.n_pointers, 5);
        assert!(unit_type.is_reference);
        if let ParsedBaseType::TypeName(type_name) = &unit_type.base_type.data {
            assert_eq!(type_name.as_str(), "MyStruct");
        } else {
            panic!("Expected TypeName variant");
        }
        assert!(diagnostics.is_empty(), "Expected no diagnostics for multiple pointers with reference");
    }

    #[test]
    fn test_parse_just_pointers() {
        let rule = ParsedUnitTypeRule {};
        let tokens = vec![
            create_token(TokenType::Int),
            create_token(TokenType::Star),
            create_token(TokenType::Star),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let unit_type = result.unwrap();
        assert_eq!(unit_type.n_pointers, 2);
        assert!(!unit_type.is_reference);
        assert!(matches!(unit_type.base_type.data, ParsedBaseType::Integer));
        assert!(diagnostics.is_empty(), "Expected no diagnostics for just pointers");
    }

    #[test]
    fn test_parse_custom_type_with_pointers_and_reference() {
        let rule = ParsedUnitTypeRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("Vector".to_string())),
            create_token(TokenType::Star),
            create_token(TokenType::Star),
            create_token(TokenType::Ampersand),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let unit_type = result.unwrap();
        assert_eq!(unit_type.n_pointers, 2);
        assert!(unit_type.is_reference);
        if let ParsedBaseType::TypeName(type_name) = &unit_type.base_type.data {
            assert_eq!(type_name.as_str(), "Vector");
        } else {
            panic!("Expected TypeName variant");
        }
        assert!(diagnostics.is_empty(), "Expected no diagnostics for custom type with pointers and reference");
    }

    #[test]
    fn test_parse_invalid_token() {
        let rule = ParsedUnitTypeRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(123)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_none());
        // No diagnostics expected here since the rule simply returns None for invalid tokens
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_parse_just_reference_without_base_type() {
        let rule = ParsedUnitTypeRule {};
        let tokens = vec![
            create_token(TokenType::Ampersand),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_just_pointer_without_base_type() {
        let rule = ParsedUnitTypeRule {};
        let tokens = vec![
            create_token(TokenType::Star),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_long_custom_type_name() {
        let rule = ParsedUnitTypeRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("VeryLongCustomTypeNameThatIsStillValid".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let unit_type = result.unwrap();
        assert_eq!(unit_type.n_pointers, 0);
        assert!(!unit_type.is_reference);
        if let ParsedBaseType::TypeName(type_name) = &unit_type.base_type.data {
            assert_eq!(type_name.as_str(), "VeryLongCustomTypeNameThatIsStillValid");
        } else {
            panic!("Expected TypeName variant");
        }
        assert!(diagnostics.is_empty(), "Expected no diagnostics for long custom type name");
    }

    #[test]
    fn test_parse_excessive_pointers() {
        let rule = ParsedUnitTypeRule {};
        let mut tokens = vec![create_token(TokenType::Int)];
        // Add 10 pointer levels
        for _ in 0..10 {
            tokens.push(create_token(TokenType::Star));
        }
        tokens.push(create_token(TokenType::EOF));
        
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let unit_type = result.unwrap();
        assert_eq!(unit_type.n_pointers, 10);
        assert!(!unit_type.is_reference);
        assert!(matches!(unit_type.base_type.data, ParsedBaseType::Integer));
        assert!(diagnostics.is_empty(), "Expected no diagnostics for excessive pointers");
    }
}
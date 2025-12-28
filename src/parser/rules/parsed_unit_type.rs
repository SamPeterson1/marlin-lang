use std::fmt;
use std::rc::Rc;

use crate::ast::{ParsedType, ParsedTypeEnum};
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::lexer::token::{Positioned, TokenType};

pub struct ParsedUnitTypeRule {}

impl fmt::Display for ParsedUnitTypeRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParsedUnitType")
    }
}

impl ParseRule<ParsedType> for ParsedUnitTypeRule {
    fn check_match(&self, cursor: ParserCursor) -> bool {
        cursor.try_match(&[
            TokenType::Int,
            TokenType::Double,
            TokenType::Bool,
            TokenType::Char,
            TokenType::Void,
            TokenType::AnyIdentifier,
        ]).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<ParsedType> {
        parser.begin_range();
        let cur= parser.next();

        let mut base_type = match cur.value {
            TokenType::Int => ParsedType::new(ParsedTypeEnum::Integer, *cur.get_position()),
            TokenType::Double => ParsedType::new(ParsedTypeEnum::Double, *cur.get_position()),
            TokenType::Bool => ParsedType::new(ParsedTypeEnum::Boolean, *cur.get_position()),
            TokenType::Char => ParsedType::new(ParsedTypeEnum::Char, *cur.get_position()),
            TokenType::Void => ParsedType::new(ParsedTypeEnum::Void, *cur.get_position()),
            TokenType::Identifier(ref type_name) => {
                ParsedType::new(ParsedTypeEnum::TypeName(type_name.to_string()), *cur.get_position())
            }
            _ => {
                return None;
            }
        };

        while parser.try_consume(TokenType::Star).is_some() {
            base_type = ParsedType::new(ParsedTypeEnum::Pointer(Box::new(base_type)), parser.current_range())
        }

        if parser.try_consume(TokenType::Ampersand).is_some() {
            base_type = ParsedType::new(ParsedTypeEnum::Reference(Box::new(base_type)), parser.current_range())
        }

        parser.end_range();

        Some(base_type)
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
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        assert!(matches!(parsed_type.parsed_type, ParsedTypeEnum::Integer));
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
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        assert!(matches!(parsed_type.parsed_type, ParsedTypeEnum::Double));
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
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        assert!(matches!(parsed_type.parsed_type, ParsedTypeEnum::Boolean));
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
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        assert!(matches!(parsed_type.parsed_type, ParsedTypeEnum::Char));
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
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        if let ParsedTypeEnum::TypeName(type_name) = &parsed_type.parsed_type {
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
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        if let ParsedTypeEnum::Pointer(inner) = &parsed_type.parsed_type {
            assert!(matches!(inner.parsed_type, ParsedTypeEnum::Integer));
        } else {
            panic!("Expected Pointer variant");
        }
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
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        
        // Unwrap three levels of pointers
        if let ParsedTypeEnum::Pointer(level1) = &parsed_type.parsed_type {
            if let ParsedTypeEnum::Pointer(level2) = &level1.parsed_type {
                if let ParsedTypeEnum::Pointer(level3) = &level2.parsed_type {
                    assert!(matches!(level3.parsed_type, ParsedTypeEnum::Char));
                } else {
                    panic!("Expected third Pointer level");
                }
            } else {
                panic!("Expected second Pointer level");
            }
        } else {
            panic!("Expected first Pointer level");
        }
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
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        if let ParsedTypeEnum::Reference(inner) = &parsed_type.parsed_type {
            assert!(matches!(inner.parsed_type, ParsedTypeEnum::Double));
        } else {
            panic!("Expected Reference variant");
        }
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
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        
        // Should be Reference(Pointer(Bool))
        if let ParsedTypeEnum::Reference(ref_inner) = &parsed_type.parsed_type {
            if let ParsedTypeEnum::Pointer(ptr_inner) = &ref_inner.parsed_type {
                assert!(matches!(ptr_inner.parsed_type, ParsedTypeEnum::Boolean));
            } else {
                panic!("Expected Pointer inside Reference");
            }
        } else {
            panic!("Expected Reference variant");
        }
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
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        
        // Should be Reference(Pointer(Pointer(Pointer(Pointer(Pointer(TypeName))))))
        if let ParsedTypeEnum::Reference(ref_inner) = &parsed_type.parsed_type {
            let mut current = &ref_inner.parsed_type;
            for _ in 0..5 {
                if let ParsedTypeEnum::Pointer(ptr_inner) = current {
                    current = &ptr_inner.parsed_type;
                } else {
                    panic!("Expected Pointer level");
                }
            }
            if let ParsedTypeEnum::TypeName(type_name) = current {
                assert_eq!(type_name.as_str(), "MyStruct");
            } else {
                panic!("Expected TypeName at base");
            }
        } else {
            panic!("Expected Reference variant");
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
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        
        // Should be Pointer(Pointer(Integer))
        if let ParsedTypeEnum::Pointer(level1) = &parsed_type.parsed_type {
            if let ParsedTypeEnum::Pointer(level2) = &level1.parsed_type {
                assert!(matches!(level2.parsed_type, ParsedTypeEnum::Integer));
            } else {
                panic!("Expected second Pointer level");
            }
        } else {
            panic!("Expected first Pointer level");
        }
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
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        
        // Should be Reference(Pointer(Pointer(TypeName)))
        if let ParsedTypeEnum::Reference(ref_inner) = &parsed_type.parsed_type {
            if let ParsedTypeEnum::Pointer(ptr1) = &ref_inner.parsed_type {
                if let ParsedTypeEnum::Pointer(ptr2) = &ptr1.parsed_type {
                    if let ParsedTypeEnum::TypeName(type_name) = &ptr2.parsed_type {
                        assert_eq!(type_name.as_str(), "Vector");
                    } else {
                        panic!("Expected TypeName at base");
                    }
                } else {
                    panic!("Expected second Pointer level");
                }
            } else {
                panic!("Expected first Pointer level");
            }
        } else {
            panic!("Expected Reference variant");
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
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_none());
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
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
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
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
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
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        if let ParsedTypeEnum::TypeName(type_name) = &parsed_type.parsed_type {
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
        for _ in 0..10 {
            tokens.push(create_token(TokenType::Star));
        }
        tokens.push(create_token(TokenType::EOF));
        
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        
        // Verify we have 10 levels of pointers
        let mut current = &parsed_type.parsed_type;
        for _ in 0..10 {
            if let ParsedTypeEnum::Pointer(inner) = current {
                current = &inner.parsed_type;
            } else {
                panic!("Expected Pointer level");
            }
        }
        assert!(matches!(current, ParsedTypeEnum::Integer));
        assert!(diagnostics.is_empty(), "Expected no diagnostics for excessive pointers");
    }
}
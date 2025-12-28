use std::fmt;
use std::rc::Rc;

use crate::ast::{ParsedType, ParsedTypeEnum};
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
        let mut unit_type = parser.apply_rule(ParsedUnitTypeRule {}, "unit type", Some(ErrMsg::ExpectedType))?;


        while parser.try_consume(TokenType::LeftSquare).is_some() {
            parser.consume_or_diagnostic(TokenType::RightSquare);

            unit_type = ParsedType::new(
                ParsedTypeEnum::Array(Box::new(unit_type)),
                parser.current_range(),
            );

            if parser.try_consume(TokenType::Ampersand).is_some() {
                unit_type = ParsedType::new(
                    ParsedTypeEnum::Reference(Box::new(unit_type)),
                    parser.current_range(),
                );
            }
        }

        parser.end_range();
        Some(unit_type)
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
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        assert!(matches!(parsed_type.parsed_type, ParsedTypeEnum::Integer));
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
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        // Should be Reference(Integer)
        if let ParsedTypeEnum::Reference(inner) = &parsed_type.parsed_type {
            assert!(matches!(inner.parsed_type, ParsedTypeEnum::Integer));
        } else {
            panic!("Expected Reference variant");
        }
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
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        // Should be Array(Integer)
        if let ParsedTypeEnum::Array(inner) = &parsed_type.parsed_type {
            assert!(matches!(inner.parsed_type, ParsedTypeEnum::Integer));
        } else {
            panic!("Expected Array variant");
        }
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
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        // Should be Reference(Array(Integer))
        if let ParsedTypeEnum::Reference(ref_inner) = &parsed_type.parsed_type {
            if let ParsedTypeEnum::Array(arr_inner) = &ref_inner.parsed_type {
                assert!(matches!(arr_inner.parsed_type, ParsedTypeEnum::Integer));
            } else {
                panic!("Expected Array inside Reference");
            }
        } else {
            panic!("Expected Reference variant");
        }
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
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        // Should be Array(Reference(Bool))
        if let ParsedTypeEnum::Array(arr_inner) = &parsed_type.parsed_type {
            if let ParsedTypeEnum::Reference(ref_inner) = &arr_inner.parsed_type {
                assert!(matches!(ref_inner.parsed_type, ParsedTypeEnum::Boolean));
            } else {
                panic!("Expected Reference inside Array");
            }
        } else {
            panic!("Expected Array variant");
        }
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
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        // Should be Array(Array(Array(Integer)))
        if let ParsedTypeEnum::Array(level1) = &parsed_type.parsed_type {
            if let ParsedTypeEnum::Array(level2) = &level1.parsed_type {
                if let ParsedTypeEnum::Array(level3) = &level2.parsed_type {
                    assert!(matches!(level3.parsed_type, ParsedTypeEnum::Integer));
                } else {
                    panic!("Expected third Array level");
                }
            } else {
                panic!("Expected second Array level");
            }
        } else {
            panic!("Expected first Array level");
        }
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
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        // Should be Reference(Array(Array(Reference(Array(Bool)))))
        if let ParsedTypeEnum::Reference(ref1) = &parsed_type.parsed_type {
            if let ParsedTypeEnum::Array(arr1) = &ref1.parsed_type {
                if let ParsedTypeEnum::Array(arr2) = &arr1.parsed_type {
                    if let ParsedTypeEnum::Reference(ref2) = &arr2.parsed_type {
                        if let ParsedTypeEnum::Array(arr3) = &ref2.parsed_type {
                            assert!(matches!(arr3.parsed_type, ParsedTypeEnum::Boolean));
                        } else {
                            panic!("Expected innermost Array");
                        }
                    } else {
                        panic!("Expected Reference");
                    }
                } else {
                    panic!("Expected second Array");
                }
            } else {
                panic!("Expected first Array");
            }
        } else {
            panic!("Expected outermost Reference");
        }
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
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        if let ParsedTypeEnum::TypeName(type_name) = &parsed_type.parsed_type {
            assert_eq!(type_name.as_str(), "MyStruct");
        } else {
            panic!("Expected TypeName variant");
        }
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
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        // Should be Reference(TypeName)
        if let ParsedTypeEnum::Reference(inner) = &parsed_type.parsed_type {
            if let ParsedTypeEnum::TypeName(type_name) = &inner.parsed_type {
                assert_eq!(type_name.as_str(), "Person");
            } else {
                panic!("Expected TypeName inside Reference");
            }
        } else {
            panic!("Expected Reference variant");
        }
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
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        // Should be Reference(Array(TypeName))
        if let ParsedTypeEnum::Reference(ref_inner) = &parsed_type.parsed_type {
            if let ParsedTypeEnum::Array(arr_inner) = &ref_inner.parsed_type {
                if let ParsedTypeEnum::TypeName(type_name) = &arr_inner.parsed_type {
                    assert_eq!(type_name.as_str(), "Person");
                } else {
                    panic!("Expected TypeName inside Array");
                }
            } else {
                panic!("Expected Array inside Reference");
            }
        } else {
            panic!("Expected Reference variant");
        }
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
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        // Should be Reference(Array(Array(Reference(TypeName))))
        if let ParsedTypeEnum::Reference(ref1) = &parsed_type.parsed_type {
            if let ParsedTypeEnum::Array(arr1) = &ref1.parsed_type {
                if let ParsedTypeEnum::Array(arr2) = &arr1.parsed_type {
                    if let ParsedTypeEnum::Reference(ref2) = &arr2.parsed_type {
                        if let ParsedTypeEnum::TypeName(type_name) = &ref2.parsed_type {
                            assert_eq!(type_name.as_str(), "Matrix");
                        } else {
                            panic!("Expected TypeName");
                        }
                    } else {
                        panic!("Expected inner Reference");
                    }
                } else {
                    panic!("Expected second Array");
                }
            } else {
                panic!("Expected first Array");
            }
        } else {
            panic!("Expected outer Reference");
        }
        assert!(diagnostics.is_empty(), "Expected no diagnostics for reference custom type with arrays");
    }

    #[test]
    fn test_parse_missing_unit_type() {
        let rule = ParsedTypeRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
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
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        // Should still be Array(Integer) despite error
        if let ParsedTypeEnum::Array(inner) = &parsed_type.parsed_type {
            assert!(matches!(inner.parsed_type, ParsedTypeEnum::Integer));
        } else {
            panic!("Expected Array variant");
        }
        
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
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        // Should be Array(Array(Double))
        if let ParsedTypeEnum::Array(level1) = &parsed_type.parsed_type {
            if let ParsedTypeEnum::Array(level2) = &level1.parsed_type {
                assert!(matches!(level2.parsed_type, ParsedTypeEnum::Double));
            } else {
                panic!("Expected second Array level");
            }
        } else {
            panic!("Expected first Array level");
        }
        
        assert!(!diagnostics.is_empty(), "Expected diagnostics for missing right brackets");
        assert_eq!(diagnostics.len(), 2);
        assert!(diagnostics.iter().all(|d| d.message.contains("']'")));
    }

    #[test]
    fn test_parse_complex_type_example() {
        let rule = ParsedTypeRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("HashMap".to_string())),
            create_token(TokenType::Ampersand),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::RightSquare),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::RightSquare),
            create_token(TokenType::Ampersand),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::RightSquare),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        // Should be Array(Reference(Array(Array(Reference(TypeName)))))
        if let ParsedTypeEnum::Array(arr1) = &parsed_type.parsed_type {
            if let ParsedTypeEnum::Reference(ref1) = &arr1.parsed_type {
                if let ParsedTypeEnum::Array(arr2) = &ref1.parsed_type {
                    if let ParsedTypeEnum::Array(arr3) = &arr2.parsed_type {
                        if let ParsedTypeEnum::Reference(ref2) = &arr3.parsed_type {
                            if let ParsedTypeEnum::TypeName(type_name) = &ref2.parsed_type {
                                assert_eq!(type_name.as_str(), "HashMap");
                            } else {
                                panic!("Expected TypeName");
                            }
                        } else {
                            panic!("Expected inner Reference");
                        }
                    } else {
                        panic!("Expected third Array");
                    }
                } else {
                    panic!("Expected second Array");
                }
            } else {
                panic!("Expected Reference");
            }
        } else {
            panic!("Expected outermost Array");
        }
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
            let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
            
            let result = rule.parse(&mut parser);
            
            assert!(result.is_some(), "Failed to parse {:?}", primitive_type);
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
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let parsed_type = result.unwrap();
        
        // Verify we have 4 levels of arrays
        let mut current = &parsed_type.parsed_type;
        for _ in 0..4 {
            if let ParsedTypeEnum::Array(inner) = current {
                current = &inner.parsed_type;
            } else {
                panic!("Expected Array level");
            }
        }
        assert!(matches!(current, ParsedTypeEnum::Integer));
        assert!(diagnostics.is_empty(), "Expected no diagnostics for array sequence");
    }
}
use std::{fmt, rc::Rc};

use crate::{ast::{ASTWrapper, parsed_type::{ParsedBaseType, ParsedUnitModifier, ParsedUnitType}}, parser::{ExprParser, ParseRule, ParserCursor, TokenCursor}, token::{TokenType, TokenValue}};

pub struct ParsedUnitTypeRule {}

impl fmt::Display for ParsedUnitTypeRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParsedUnitType")
    }
}

impl ParseRule<ASTWrapper<ParsedUnitType>> for ParsedUnitTypeRule {
    fn check_match(&self, cursor: ParserCursor) -> bool {
        let cur = cursor.cur();

        matches!(
            &cur.token_type,
            TokenType::Int
                | TokenType::Double
                | TokenType::Bool
                | TokenType::Identifier
        )
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<ASTWrapper<ParsedUnitType>> {
        let cur = parser.next();

        let base_type = match (&cur.token_type, &cur.value) {
            (TokenType::Int, TokenValue::None) => ASTWrapper::new_parsed_base_type(ParsedBaseType::Integer, cur.position),
            (TokenType::Double, TokenValue::None) => ASTWrapper::new_parsed_base_type(ParsedBaseType::Double, cur.position),
            (TokenType::Bool, TokenValue::None) => ASTWrapper::new_parsed_base_type(ParsedBaseType::Boolean, cur.position),
            (TokenType::Identifier, TokenValue::String(type_name)) => {
                ASTWrapper::new_parsed_base_type(ParsedBaseType::TypeName(Rc::new(type_name.to_string())), cur.position)
            }
            _ => {
                return None;
            }
        };

        let is_reference = parser.try_consume(TokenType::Ampersand).is_some();
        let mut n_pointers = 0;

        if !is_reference {
            while parser.try_consume(TokenType::Star).is_some() {
                n_pointers += 1;
            }
        }

        let modifier = if is_reference {
            ParsedUnitModifier::Reference
        } else if n_pointers > 0 {
            ParsedUnitModifier::Pointer(n_pointers)
        } else {
            ParsedUnitModifier::None
        };

        Some(ASTWrapper::new_parsed_unit_type(base_type, modifier, cur.position))
    }
}
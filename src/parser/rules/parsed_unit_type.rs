use std::{array, fmt, rc::Rc};

use crate::{ast::{ASTWrapper, new_array_expr::NewArrayExpr, parsed_type::{ParsedBaseType, ParsedType, ParsedUnitType}}, parser::{ExprParser, ParseRule, ParserCursor, TokenCursor, diagnostic::{self, ErrMsg}}, token::{Position, PositionRange, TokenType, TokenValue}};

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

        Some(ASTWrapper::new_parsed_unit_type(base_type, is_reference, cur.position))
    }
}
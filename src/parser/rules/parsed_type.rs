use std::{array, fmt, rc::Rc};

use crate::{ast::{ASTWrapper, new_array_expr::NewArrayExpr, parsed_type::{ParsedType, ParsedUnitType}}, parser::{ExprParser, ParseRule, ParserCursor, TokenCursor, diagnostic::{self, ErrMsg}}, token::{Position, PositionRange, TokenType, TokenValue}};

pub struct ParsedTypeRule {}

impl fmt::Display for ParsedTypeRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParsedType")
    }
}

impl ParseRule<ASTWrapper<ParsedType>> for ParsedTypeRule {
    fn check_match(&self, _cursor: ParserCursor) -> bool {
        true
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<ASTWrapper<ParsedType>> {
        let start_pos = parser.cur().position;

        let is_reference = parser.try_consume(TokenType::Ampersand).is_some();
        let cur = parser.cur();

        let unit_type = match (&cur.token_type, &cur.value) {
            (TokenType::Int, TokenValue::None) => ParsedUnitType::Integer,
            (TokenType::Double, TokenValue::None) => ParsedUnitType::Double,
            (TokenType::Bool, TokenValue::None) => ParsedUnitType::Boolean,
            (TokenType::Identifier, TokenValue::String(type_name)) => {
                ParsedUnitType::TypeName(Rc::new(type_name.to_string()))
            }
            _ => {
                parser.push_diagnostic(ErrMsg::ExpectedTypeNameOrIdentifier.make_diagnostic(cur.position));
                return None;
            }
        };

        parser.next();

        let mut array_dimension: u32 = 0;

        while parser.try_consume(TokenType::LeftSquare).is_some() {
            array_dimension += 1;
            parser.consume_or_diagnostic(TokenType::RightSquare);
        }

        let position = PositionRange::concat(&start_pos, &parser.prev().position);

        if is_reference && array_dimension > 0 {
            parser.push_diagnostic(ErrMsg::CannotReferenceArrayType.make_diagnostic(position));
            return None;
        }

        let parsed_type = if is_reference {
            ParsedType::Reference(unit_type)
        } else if array_dimension > 0 {
            ParsedType::Array(unit_type, array_dimension)
        } else {
            ParsedType::Unit(unit_type)
        };

        Some(ASTWrapper::new_parsed_type(parsed_type, position))
    }
}
use std::fmt;

use crate::{ast::{ASTWrapper, parsed_type::ParsedType}, parser::{ExprParser, ParseRule, ParserCursor, TokenCursor, diagnostic::ErrMsg, rules::parsed_unit_type::ParsedUnitTypeRule}, token::{PositionRange, TokenType}};

pub struct ParsedTypeRule {}

impl fmt::Display for ParsedTypeRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParsedType")
    }
}

impl ParseRule<ASTWrapper<ParsedType>> for ParsedTypeRule {
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

    fn parse(&self, parser: &mut ExprParser) -> Option<ASTWrapper<ParsedType>> {
        let start_pos = parser.cur().position;
        let unit_type = parser.apply_rule(ParsedUnitTypeRule {}, "unit type", Some(ErrMsg::ExpectedTypeNameOrIdentifier))?;

        let mut array_dimension: u32 = 0;

        while parser.try_consume(TokenType::LeftSquare).is_some() {
            array_dimension += 1;
            parser.consume_or_diagnostic(TokenType::RightSquare);
        }

        let is_reference = parser.try_consume(TokenType::Ampersand).is_some();

        let position = PositionRange::concat(&start_pos, &parser.prev().position);

        Some(ASTWrapper::new_parsed_type(is_reference, unit_type, array_dimension, position))
    }
}
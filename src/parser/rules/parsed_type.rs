use std::fmt;

use crate::ast::parsed_type::ParsedType;
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
        let unit_type = parser.apply_rule(ParsedUnitTypeRule {}, "unit type", Some(ErrMsg::ExpectedTypeNameOrIdentifier))?;

        let mut array_dimension: u32 = 0;

        while parser.try_consume(TokenType::LeftSquare).is_some() {
            array_dimension += 1;
            parser.consume_or_diagnostic(TokenType::RightSquare);
        }

        let is_reference = parser.try_consume(TokenType::Ampersand).is_some();

        Some(ParsedType::new(is_reference, unit_type, array_dimension, parser.end_range()))
    }
}
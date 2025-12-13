use std::fmt;
use std::rc::Rc;

use crate::ast::parsed_type::{ParsedBaseType, ParsedUnitModifier, ParsedUnitType};
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

        Some(ParsedUnitType::new(base_type, modifier, parser.end_range()))
    }
}
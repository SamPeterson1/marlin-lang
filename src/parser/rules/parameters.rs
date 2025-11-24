use std::fmt;

use crate::{ast::{ASTWrapper, parameters::Parameters}, parser::{ExprParser, ParseRule, ParserCursor, TokenCursor, rules::parsed_type::ParsedTypeRule}, token::{PositionRange, TokenType}};

pub struct ParametersRule {}

impl fmt::Display for ParametersRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parameters")
    }
}

impl ParseRule<ASTWrapper<Parameters>> for ParametersRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::LeftParen).is_some()
    }
    
    fn parse(&self, parser: &mut ExprParser) -> Option<ASTWrapper<Parameters>> {
        let left_paren = parser.try_consume(TokenType::LeftParen)?;

        let mut parameters = Vec::new();
        
        if let Some(parsed_type) = parser.apply_rule(ParsedTypeRule {}, "first argument", None) {
            let identifier = parser.try_consume(TokenType::Identifier)?;

            parameters.push((parsed_type, identifier.get_string().to_string()));
        }

        let right_paren = parser.consume_or_diagnostic(TokenType::RightParen);

        let position = PositionRange::concat(&left_paren.position, &parser.prev().position);

        Some(ASTWrapper::new_parameters(parameters, position))
    }
}
use std::fmt;

use crate::ast::parameters::Parameters;
use crate::diagnostic::ErrMsg;
use crate::logger::Log;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::parsed_type::ParsedTypeRule;
use crate::lexer::token::TokenType;

pub struct ParametersRule {}

impl fmt::Display for ParametersRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parameters")
    }
}

impl ParseRule<Parameters> for ParametersRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::LeftParen).is_some()
    }
    
    fn parse(&self, parser: &mut ExprParser) -> Option<Parameters> {
        parser.begin_range();
        parser.try_consume(TokenType::LeftParen)?;

        let mut parameters = Vec::new();
        
        if let Some(parsed_type) = parser.apply_rule(ParsedTypeRule {}, "first parameter type", None) {
            let identifier = parser.try_consume(TokenType::AnyIdentifier)?;

            parameters.push((parsed_type, identifier.unwrap_identifier()));
        }

        parser.log_debug(&format!("Current token after first parameter parse: {:?}", parser.cur()));

        while let Some(_) = parser.try_consume(TokenType::Comma) {
            let parsed_type = parser.apply_rule(ParsedTypeRule {}, "parameter type", Some(ErrMsg::ExpectedType))?;
            let identifier = parser.consume_or_diagnostic(TokenType::AnyIdentifier)?;

            parameters.push((parsed_type, identifier.unwrap_identifier()));
        }

        parser.consume_or_diagnostic(TokenType::RightParen);

        Some(Parameters::new(parameters, parser.end_range()))
    }
}
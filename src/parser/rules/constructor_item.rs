use std::fmt;

use crate::{ast::{ASTWrapper, constructor_item::ConstructorItem}, parser::{ExprParser, ParseRule, ParserCursor, TokenCursor, diagnostic::ErrMsg, rules::{block::BlockRule, parameters::ParametersRule,}}, token::{PositionRange, Positioned, TokenType}};

pub struct ConstructorRule {}

impl fmt::Display for ConstructorRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Constructor")
    }
}

impl ParseRule<ASTWrapper<ConstructorItem>> for ConstructorRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::DollarSign).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<ASTWrapper<ConstructorItem>> {
        let dollar_token = parser.try_consume(TokenType::DollarSign)?;

        let parameters = parser.apply_rule(ParametersRule {}, "constructor parameters", Some(ErrMsg::ExpectedParameters))?;
        let body = parser.apply_rule(BlockRule {}, "constructor body", Some(ErrMsg::ExpectedBlock))?;        
        
        let position = PositionRange::concat(&dollar_token.position, body.get_position());

        Some(ASTWrapper::new_constructor_item(parameters, body, position))
    }
}
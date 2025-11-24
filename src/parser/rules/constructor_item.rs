use std::fmt;

use crate::{ast::{ASTWrapper, constructor_item::ConstructorItem, function_item::FunctionItem, parameters::Parameters}, logger::Log, parser::{ExprParser, ParseRule, ParserCursor, TokenCursor, diagnostic::{self, ErrMsg}, rules::{block::BlockRule, function_prototype::FunctionPrototypeRule, parameters::ParametersRule, parsed_type::ParsedTypeRule}}, token::{Position, PositionRange, Positioned, TokenType}};

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
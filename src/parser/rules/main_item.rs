use std::fmt;

use crate::{ast::{ASTWrapper, function_item::FunctionItem, main_item::MainItem}, logger::Log, parser::{ExprParser, ParseRule, ParserCursor, TokenCursor, diagnostic::{self, ErrMsg}, rules::{block::BlockRule, function_prototype::FunctionPrototypeRule, parsed_type::ParsedTypeRule}}, token::{Position, PositionRange, Positioned, TokenType}};

pub struct MainItemRule {}

impl fmt::Display for MainItemRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MainItem")
    }
}

impl ParseRule<ASTWrapper<MainItem>> for MainItemRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::Main).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<ASTWrapper<MainItem>> {
        let main_token = parser.try_consume(TokenType::Main)?;

        let block = parser.apply_rule(BlockRule {}, "main block", Some(ErrMsg::ExpectedBlock))?;
        let position = PositionRange::concat(&main_token.position, block.get_position());

        Some(ASTWrapper::new_main(block, position))
    }
}
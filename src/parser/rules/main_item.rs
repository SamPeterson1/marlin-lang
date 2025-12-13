use std::fmt;

use crate::ast::main_item::MainItem;
use crate::diagnostic::ErrMsg;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::block::BlockRule;
use crate::lexer::token::TokenType;

pub struct MainItemRule {}

impl fmt::Display for MainItemRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MainItem")
    }
}

impl ParseRule<MainItem> for MainItemRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::Main).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<MainItem> {
        parser.begin_range();
        parser.try_consume(TokenType::Main)?;

        let block = parser.apply_rule(BlockRule {}, "main block", Some(ErrMsg::ExpectedBlock))?;

        Some(MainItem::new(block, parser.end_range()))
    }
}
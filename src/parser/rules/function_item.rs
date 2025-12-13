use std::fmt;

use crate::ast::function_item::FunctionItem;
use crate::diagnostic::ErrMsg;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::{block::BlockRule, parameters::ParametersRule, parsed_type::ParsedTypeRule};
use crate::lexer::token::TokenType;

pub struct FunctionRule;

impl fmt::Display for FunctionRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Function")
    }
}

impl ParseRule<FunctionItem> for FunctionRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::Fn).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<FunctionItem> {
        parser.begin_range();
        parser.try_consume(TokenType::Fn)?;

        let name = parser.consume_or_diagnostic(TokenType::AnyIdentifier)?.unwrap_identifier();

        let parameters = parser.apply_rule(ParametersRule {}, "function parameters", Some(ErrMsg::ExpectedParameters))?;

        parser.consume_or_diagnostic(TokenType::Arrow);

        let ret_type = parser.apply_rule(ParsedTypeRule {}, "return type", None)?;

        let block = parser.apply_rule(BlockRule {}, "function body", Some(ErrMsg::ExpectedBlock))?;

        Some(FunctionItem::new(name, parameters, ret_type, block, parser.end_range()))
    }
}
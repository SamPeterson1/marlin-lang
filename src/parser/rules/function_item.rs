use std::fmt;

use crate::{ast::{ASTWrapper, function_item::FunctionItem}, parser::{ExprParser, ParseRule, ParserCursor, TokenCursor, diagnostic::ErrMsg, rules::{block::BlockRule, parameters::ParametersRule, parsed_type::ParsedTypeRule}}, token::{PositionRange, TokenType}};

pub struct FunctionRule;

impl fmt::Display for FunctionRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Function")
    }
}

impl ParseRule<ASTWrapper<FunctionItem>> for FunctionRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::Fn).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<ASTWrapper<FunctionItem>> {
        let fn_token = parser.try_consume(TokenType::Fn)?;

        let name = parser.consume_or_diagnostic(TokenType::Identifier)?.get_string().to_string();

        let parameters = parser.apply_rule(ParametersRule {}, "function parameters", Some(ErrMsg::ExpectedParameters))?;

        parser.consume_or_diagnostic(TokenType::Arrow);

        let ret_type = parser.apply_rule(ParsedTypeRule {}, "return type", None)?;

        let block = parser.apply_rule(BlockRule {}, "function body", Some(ErrMsg::ExpectedBlock))?;
        let position = PositionRange::concat(&fn_token.position, &block.position);

        Some(ASTWrapper::new_function_item(name, parameters, ret_type, block, position))
    }
}
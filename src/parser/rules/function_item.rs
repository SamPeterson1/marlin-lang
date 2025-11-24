use std::fmt;

use crate::{ast::{ASTWrapper, function_item::FunctionItem}, logger::Log, parser::{ExprParser, ParseRule, ParserCursor, TokenCursor, diagnostic::{self, ErrMsg}, rules::{block::BlockRule, function_prototype::FunctionPrototypeRule, parsed_type::ParsedTypeRule}}, token::{Position, PositionRange, TokenType}};

pub struct FunctionRule {}

impl fmt::Display for FunctionRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Function")
    }
}

impl ParseRule<ASTWrapper<FunctionItem>> for FunctionRule {
    fn check_match(&self, cursor: ParserCursor) -> bool {
        (FunctionPrototypeRule {}).check_match(cursor)
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<ASTWrapper<FunctionItem>> {
        let function_prototype = parser.apply_rule(FunctionPrototypeRule {}, "function prototype", None)?;

        parser.consume_or_diagnostic(TokenType::AtSign);
        
        let src_type = parser.apply_rule(ParsedTypeRule {}, "function src type", None)?;
        let src_identifier = parser.try_consume(TokenType::Identifier)?.get_string().to_string();

        let block = parser.apply_rule(BlockRule {}, "function body", Some(ErrMsg::ExpectedBlock))?;

        Some(ASTWrapper::new_function_item(function_prototype, block, src_type, src_identifier))
    }
}
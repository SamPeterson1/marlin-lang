use std::fmt;

use crate::{ast::{ASTWrapper, impl_item::ImplItem}, parser::{ExprParser, ParseRule, ParserCursor, TokenCursor, diagnostic::ErrMsg, rules::{function_item::FunctionRule, parsed_type::ParsedTypeRule}}, token::{PositionRange, TokenType}};

pub struct ImplBlockRule {}

impl fmt::Display for ImplBlockRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ImplBlock")
    }
}

impl ParseRule<ASTWrapper<ImplItem>> for ImplBlockRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::Impl).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<ASTWrapper<ImplItem>> {
        let impl_token = parser.try_consume(TokenType::Impl)?;

        let impl_type = parser.apply_rule(ParsedTypeRule {}, "impl type", Some(ErrMsg::ExpectedType))?;

        parser.consume_or_diagnostic(TokenType::LeftCurly);

        let mut functions = Vec::new();

        while let Some(function) = parser.apply_rule(FunctionRule { }, "impl function", None) {
            functions.push(function);
        }

        parser.consume_or_diagnostic(TokenType::RightCurly);

        let position = PositionRange::concat(&impl_token.position, &parser.prev().position);

        Some(ASTWrapper::new_impl_item(impl_type, functions, position))
    }
}

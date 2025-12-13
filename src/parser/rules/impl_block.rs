use std::fmt;

use crate::ast::impl_item::ImplItem;
use crate::diagnostic::ErrMsg;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::{function_item::FunctionRule, parsed_type::ParsedTypeRule};
use crate::lexer::token::TokenType;

pub struct ImplBlockRule {}

impl fmt::Display for ImplBlockRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ImplBlock")
    }
}

impl ParseRule<ImplItem> for ImplBlockRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::Impl).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<ImplItem> {
        parser.begin_range();
        parser.try_consume(TokenType::Impl)?;

        let impl_type = parser.apply_rule(ParsedTypeRule {}, "impl type", Some(ErrMsg::ExpectedType))?;

        parser.consume_or_diagnostic(TokenType::LeftCurly);

        let mut functions = Vec::new();

        while let Some(function) = parser.apply_rule(FunctionRule { }, "impl function", None) {
            functions.push(function);
        }

        parser.consume_or_diagnostic(TokenType::RightCurly);

        Some(ImplItem::new(impl_type, functions, parser.end_range()))
    }
}

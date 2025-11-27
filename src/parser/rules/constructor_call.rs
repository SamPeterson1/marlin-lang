use std::fmt;

use crate::{ast::{ASTWrapper, constructor_call::ConstructorCallExpr}, parser::{ExprParser, ParseRule, ParserCursor, TokenCursor, diagnostic::ErrMsg, rules::arguments::ArgumentsRule}, token::{PositionRange, TokenType}};

pub struct ConstructorCallRule {}

impl fmt::Display for ConstructorCallRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ConstructorCall")
    }
}

impl ParseRule<ASTWrapper<ConstructorCallExpr>> for ConstructorCallRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::New);
        cursor.try_consume(TokenType::DollarSign).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<ASTWrapper<ConstructorCallExpr>> {
        let first_token = parser.cur();

        let is_heap = parser.try_consume(TokenType::New).is_some();
        parser.try_consume(TokenType::DollarSign);

        let type_name = parser.consume_or_diagnostic(TokenType::Identifier)?.get_string().to_string();

        let arguments = parser.apply_rule(ArgumentsRule {}, "constructor arguments", Some(ErrMsg::ExpectedArguments))?;
        
        let position = PositionRange::concat(&first_token.position, &parser.prev().position);

        Some(ASTWrapper::new_constructor_call(type_name, arguments, is_heap, position))
    }
}
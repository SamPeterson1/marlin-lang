use std::fmt;

use crate::ast::constructor_call::ConstructorCallExpr;
use crate::diagnostic::ErrMsg;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::arguments::ArgumentsRule;
use crate::lexer::token::TokenType;

pub struct ConstructorCallRule {}

impl fmt::Display for ConstructorCallRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ConstructorCall")
    }
}

impl ParseRule<ConstructorCallExpr> for ConstructorCallRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::New);
        cursor.try_consume(TokenType::DollarSign).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<ConstructorCallExpr> {
        parser.begin_range();

        let is_heap = parser.try_consume(TokenType::New).is_some();
        parser.try_consume(TokenType::DollarSign);

        let type_name = parser.consume_or_diagnostic(TokenType::AnyIdentifier)?.unwrap_identifier();

        let arguments = parser.apply_rule(ArgumentsRule {}, "constructor arguments", Some(ErrMsg::ExpectedArguments))?;
        
        Some(ConstructorCallExpr::new(type_name, arguments, is_heap, parser.end_range()))
    }
}
use std::fmt;

use crate::{ast::{ASTWrapper, break_expr::BreakExpr}, parser::{ExprParser, ParseRule, ParserCursor, TokenCursor, diagnostic::{self, ErrMsg}, rules::expr::ExprRule}, token::{Position, PositionRange, TokenType}};

pub struct BreakRule {}

impl fmt::Display for BreakRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Break")
    }
}

impl ParseRule<ASTWrapper<BreakExpr>> for BreakRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::Break).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<ASTWrapper<BreakExpr>> {
        let break_token = parser.try_consume(TokenType::Break)?;

        let expr = parser.apply_rule(ExprRule {}, "break expression", Some(ErrMsg::ExpectedExpression));
        parser.consume_or_diagnostic(TokenType::Semicolon);

        let position = PositionRange::concat(&break_token.position, &parser.prev().position);

        Some(ASTWrapper::new_break(expr?, position))
    }
}
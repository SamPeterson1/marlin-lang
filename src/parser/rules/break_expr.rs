use std::fmt;

use crate::{expr::{break_expr::BreakExpr, put_char_expr::PutCharExpr}, parser::{self, ExprParser, ParseRule, diagnostic, rules::expr::ExprRule}, token::{Position, PositionRange, TokenType}};

pub struct BreakRule {}

impl fmt::Display for BreakRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Break")
    }
}

impl ParseRule<BreakExpr> for BreakRule {
    fn parse(&self, parser: &mut ExprParser) -> Option<BreakExpr> {
        let break_token = parser.advance();

        let expr = parser.apply_rule(ExprRule {});

        parser.log_parse_result(&expr, "break expression");
        parser.consume_or_diagnostic(TokenType::Semicolon, diagnostic::err_expected_token(PositionRange::new(Position::new(0, 0)), TokenType::Semicolon));

        let position = PositionRange::concat(&break_token.position, &parser.prev().position);

        Some(BreakExpr::new(expr?, position))
    }
}
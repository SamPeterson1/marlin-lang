use std::fmt;

use crate::{ast::{ASTWrapper, exit_expr::{ExitExpr, ExitType}}, parser::{ExprParser, ParseRule, ParserCursor, TokenCursor, diagnostic::ErrMsg, rules::expr::ExprRule}, token::{PositionRange, TokenType}};

pub struct ExitRule {}

impl fmt::Display for ExitRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Exit")
    }
}

impl ParseRule<ASTWrapper<ExitExpr>> for ExitRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_match(&[TokenType::Break, TokenType::Return, TokenType::Result]).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<ASTWrapper<ExitExpr>> {
        let start_position = parser.cur().position;

        let exit_type = match parser.try_consume_match(&[TokenType::Break, TokenType::Return, TokenType::Result])?.token_type {
            TokenType::Break => ExitType::Break,
            TokenType::Return => ExitType::Return,
            TokenType::Result => ExitType::Result,
            _ => return None,
        };

        let expr = if parser.try_consume(TokenType::Semicolon).is_none() {
            let result = parser.apply_rule(ExprRule {}, "exit expression", Some(ErrMsg::ExpectedExpression))?;
            parser.consume_or_diagnostic(TokenType::Semicolon);

            Some(result)
        } else {
            None
        };

        let position = PositionRange::concat(&start_position, &parser.prev().position);
        Some(ASTWrapper::new_exit(exit_type, expr, position))
    }
}
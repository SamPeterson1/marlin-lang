use std::fmt;

use crate::ast::exit_expr::{ExitExpr, ExitType};
use crate::diagnostic::ErrMsg;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::expr::ExprRule;
use crate::lexer::token::TokenType;

pub struct ExitRule {}

impl fmt::Display for ExitRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Exit")
    }
}

impl ParseRule<ExitExpr> for ExitRule {
    fn check_match(&self, cursor: ParserCursor) -> bool {
        cursor.try_match(&[TokenType::Break, TokenType::Return, TokenType::Result]).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<ExitExpr> {
        parser.begin_range();

        let exit_type = match parser.try_consume_match(&[TokenType::Break, TokenType::Return, TokenType::Result])?.value {
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

        Some(ExitExpr::new(exit_type, expr, parser.end_range()))
    }
}
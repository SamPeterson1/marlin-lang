use std::fmt;

use crate::ast::delete_expr::DeleteExpr;
use crate::diagnostic::ErrMsg;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::expr::ExprRule;
use crate::lexer::token::TokenType;

pub struct DeleteRule {}

impl fmt::Display for DeleteRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Delete")
    }
}

impl ParseRule<DeleteExpr> for DeleteRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::Delete).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<DeleteExpr> {
        parser.begin_range();

        parser.try_consume(TokenType::Delete)?;
        let expr = parser.apply_rule(ExprRule {}, "delete expression", Some(ErrMsg::ExpectedExpression))?;
        parser.consume_or_diagnostic(TokenType::Semicolon);

        Some(DeleteExpr::new(expr, parser.end_range()))
    }
}
use std::fmt;

use crate::{ast::{ASTWrapper, put_char_expr::PutCharExpr}, parser::{ExprParser, ParseRule, ParserCursor, TokenCursor, diagnostic::ErrMsg, rules::expr::ExprRule}, token::{Position, PositionRange, TokenType}};

pub struct PutcRule {}

impl fmt::Display for PutcRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Putc")
    }
}

impl ParseRule<ASTWrapper<PutCharExpr>> for PutcRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::Putc).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<ASTWrapper<PutCharExpr>> {
        let putc_token = parser.try_consume(TokenType::Putc)?;
        
        let expr = parser.apply_rule(ExprRule {}, "putc expression", Some(ErrMsg::ExpectedExpression))?;
        
        parser.consume_or_diagnostic(TokenType::Semicolon)?;
        
        let position = PositionRange::concat(&putc_token.position, &parser.prev().position);

        Some(ASTWrapper::new_put_char(expr, position))
    }
}
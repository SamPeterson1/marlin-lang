use std::fmt;

use crate::{ast::{ASTWrapper, get_char_expr::GetCharExpr}, logger::Log, parser::{ExprParser, ParseRule, ParserCursor, TokenCursor}, token::{Position, PositionRange, TokenType}};

pub struct GetcRule {}

impl fmt::Display for GetcRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Getc")
    }
}

impl ParseRule<ASTWrapper<GetCharExpr>> for GetcRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::Getc).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<ASTWrapper<GetCharExpr>> {
        let getc_token = parser.try_consume(TokenType::Getc)?;
    
        Some(ASTWrapper::new_get_char(getc_token.position))
    }
}
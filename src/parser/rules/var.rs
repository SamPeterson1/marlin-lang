use std::fmt;

use crate::ast::var_expr::VarExpr;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::lexer::token::TokenType;

pub struct VarRule {}

impl fmt::Display for VarRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Var")
    }
}

impl ParseRule<VarExpr> for VarRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::AnyIdentifier).is_some()
    }
    
    fn parse(&self, parser: &mut ExprParser) -> Option<VarExpr> {
        let identifier = parser.try_consume(TokenType::AnyIdentifier)?;

        Some(VarExpr::new(identifier.unwrap_identifier()))
    }
}

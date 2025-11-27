use std::fmt;

use crate::{ast::{ASTWrapper, var_expr::VarExpr}, parser::{ExprParser, ParseRule, ParserCursor, TokenCursor}, token::TokenType};

pub struct VarRule {}

impl fmt::Display for VarRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Var")
    }
}

impl ParseRule<ASTWrapper<VarExpr>> for VarRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::Identifier).is_some()
    }
    
    fn parse(&self, parser: &mut ExprParser) -> Option<ASTWrapper<VarExpr>> {
        let identifier = parser.try_consume(TokenType::Identifier)?;

        Some(ASTWrapper::new_var(0, identifier))
    }
}

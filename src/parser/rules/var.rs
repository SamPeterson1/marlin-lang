use std::fmt;

use crate::{ast::{ASTWrapper, lvar_expr::{VarExpr}}, logger::Log, parser::{ExprParser, ParseRule, ParserCursor, TokenCursor}, token::{PositionRange, TokenType}};

pub struct VarRule {}

impl fmt::Display for VarRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Var")
    }
}

impl ParseRule<ASTWrapper<VarExpr>> for VarRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::Ampersand);
        cursor.try_consume(TokenType::Identifier).is_some()
    }
    
    fn parse(&self, parser: &mut ExprParser) -> Option<ASTWrapper<VarExpr>> {
        let first_token = parser.cur();

        let is_reference = parser.try_consume(TokenType::Ampersand).is_some();
        let identifier = parser.try_consume(TokenType::Identifier)?;

        let position = PositionRange::concat(&first_token.position, &identifier.position);

        Some(ASTWrapper::new_var(0, identifier.get_string().to_string(), is_reference, position))
    }
}

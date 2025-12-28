use std::fmt;

use crate::ast::VarExpr;
use crate::parser::rules::path::PathRule;
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
        let path = parser.apply_rule(PathRule {}, "var expr path", None)?;

        Some(VarExpr::new(path))
    }
}

#[cfg(test)]
mod tests {

}

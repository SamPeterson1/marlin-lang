    use std::fmt;

use crate::ast::{DeclarationExpr, Scope};
use crate::diagnostic::ErrMsg;
use crate::logger::Log;
use crate::parser::rules::declaration::DeclarationRule;
use crate::parser::rules::path::PathRule;
use crate::parser::rules::program::ProgramRule;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::parsed_type::ParsedTypeRule;
use crate::lexer::token::{Located, TokenType};

pub struct ScopeRule {}

impl fmt::Display for ScopeRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Scope")
    }
}

impl ParseRule<Scope> for ScopeRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::Scope).is_some()
    }
    
    fn parse(&self, parser: &mut ExprParser) -> Option<Scope> {
        parser.begin_range();
        
        parser.consume_or_diagnostic(TokenType::Scope)?;

        let path = parser.apply_rule(PathRule {}, "scope path", None)?;
        parser.consume_or_diagnostic(TokenType::LeftCurly)?;

        let program = parser.apply_rule(ProgramRule {}, "scope program", None)?;
        
        Some(Scope::new(path, program, parser.end_range()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::{Token, TokenType, PositionRange};

}
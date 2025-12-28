use std::fmt;

use crate::ast::Path;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::lexer::token::TokenType;

pub struct PathRule {}

impl fmt::Display for PathRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Path")
    }
}

impl ParseRule<Path> for PathRule {
    fn check_match(&self, _cursor: ParserCursor) -> bool {
        true
    }
    
    fn parse(&self, parser: &mut ExprParser) -> Option<Path> {        
        let mut path = Vec::new();

        let identifier = parser.consume_or_diagnostic(TokenType::AnyIdentifier)?.unwrap_identifier();
        path.push(identifier);

        while let Some(_) = parser.try_consume(TokenType::DoubleColon) {
            let identifier = parser.consume_or_diagnostic(TokenType::AnyIdentifier)?.unwrap_identifier();
            path.push(identifier);
        }

        Some(Path::new(path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::{Token, TokenType, PositionRange};

}
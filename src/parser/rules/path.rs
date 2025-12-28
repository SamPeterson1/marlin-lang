use std::fmt;

use crate::ast::DeclarationExpr;
use crate::diagnostic::ErrMsg;
use crate::logger::Log;
use crate::parser::rules::declaration::DeclarationRule;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::parsed_type::ParsedTypeRule;
use crate::lexer::token::{Located, TokenType};

pub struct PathRule {}

impl fmt::Display for PathRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Path")
    }
}

impl ParseRule<Vec<Located<String>>> for PathRule {
    fn check_match(&self, _cursor: ParserCursor) -> bool {
        true
    }
    
    fn parse(&self, parser: &mut ExprParser) -> Option<Vec<Located<String>>> {        
        let mut path = Vec::new();

        let identifier = parser.consume_or_diagnostic(TokenType::AnyIdentifier)?.unwrap_identifier();
        path.push(identifier);

        while let Some(_) = parser.try_consume(TokenType::DoubleColon) {
            let identifier = parser.consume_or_diagnostic(TokenType::AnyIdentifier)?.unwrap_identifier();
            path.push(identifier);
        }

        Some(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::{Token, TokenType, PositionRange};

}
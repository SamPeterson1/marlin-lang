use std::fmt;

use crate::ast::Require;
use crate::parser::rules::path::PathRule;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::lexer::token::{Located, TokenType};

pub struct RequireRule {}

impl fmt::Display for RequireRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Require")
    }
}

impl RequireRule {
    fn parse_path_alias(&self, prefix: Option<Vec<Located<String>>>, parser: &mut ExprParser) -> Option<(Vec<Located<String>>, Option<Located<String>>)> {
        let path = parser.apply_rule(PathRule {}, "require path", None)?;
        
        let path = if let Some(mut prefix) = prefix {
            prefix.extend(path);
            prefix
        } else {
            path
        };

        let alias = if let Some(_) = parser.try_consume(TokenType::As) {
            let identifier = parser.consume_or_diagnostic(TokenType::AnyIdentifier)?.unwrap_identifier();
            Some(identifier)
        } else {
            None
        };

        Some((path, alias))
    }
}

impl ParseRule<Vec<Require>> for RequireRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume_match(&[TokenType::Require, TokenType::From]).is_some()
    }
    
    fn parse(&self, parser: &mut ExprParser) -> Option<Vec<Require>> {        
        let from_path = if let Some(_) = parser.try_consume(TokenType::From) {
            Some(parser.apply_rule(PathRule {}, "require from path", None)?)
        } else {
            None
        };

        let mut requires = Vec::new();

        requires.push(self.parse_path_alias(from_path.clone(), parser)?);

        while let Some(_) = parser.try_consume(TokenType::Comma) {
            requires.push(self.parse_path_alias(from_path.clone(), parser)?);
        }
        
        parser.consume_or_diagnostic(TokenType::Semicolon);

        Some(requires.into_iter().map(|(path, alias)| {
            Require {
                path,
                alias,
            }
        }).collect::<Vec<Require>>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::{Token, TokenType, PositionRange};

}
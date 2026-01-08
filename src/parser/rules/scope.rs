use std::fmt;

use crate::ast::Scope;
use crate::parser::rules::item::ItemRule;
use crate::parser::rules::path::PathRule;
use crate::parser::rules::require::RequireRule;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::lexer::token::TokenType;

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

        let mut requires = Vec::new();

        while let Some(require) = parser.apply_rule(RequireRule {}, "require", None) {
            requires.extend(require);
        }

        let mut items = Vec::new();
        let mut child_scopes = Vec::new();

        loop {
            if let Some(item) = parser.apply_rule(ItemRule {}, "item", None) {
                items.push(item);
                continue;
            }

            if let Some(scope) = parser.apply_rule(ScopeRule {}, "child scope", None) {
                child_scopes.push(scope);
                continue;
            }

            break;
        }

        parser.consume_or_diagnostic(TokenType::RightCurly)?;

        Some(Scope::new(path, requires, child_scopes, items, parser.end_range()))
    }
}
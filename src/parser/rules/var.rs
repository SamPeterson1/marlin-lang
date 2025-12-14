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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::{Token, TokenType, PositionRange, Position};
    use crate::diagnostic::Diagnostic;

    fn create_token(token_type: TokenType) -> Token {
        Token::new(token_type, PositionRange::new(Position::new(1, 1)))
    }

    #[test]
    fn test_check_match_with_identifier() {
        let rule = VarRule {};
        let tokens = vec![create_token(TokenType::Identifier("variable".to_string()))];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };

        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_check_match_fails_with_non_identifier() {
        let rule = VarRule {};
        let tokens = vec![create_token(TokenType::IntLiteral(42))];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };

        assert!(!rule.check_match(cursor));
    }

    #[test]
    fn test_parse_simple_identifier() {
        let rule = VarRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("variable".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);

        let result = rule.parse(&mut parser);

        assert!(result.is_some());
        let var_expr = result.unwrap();
        assert_eq!(var_expr.identifier.data, "variable");
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_parse_fails_with_non_identifier() {
        let rule = VarRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);

        let result = rule.parse(&mut parser);

        assert!(result.is_none());
    }

    #[test]
    fn test_parse_underscore_identifier() {
        let rule = VarRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("_private".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);

        let result = rule.parse(&mut parser);

        assert!(result.is_some());
        let var_expr = result.unwrap();
        assert_eq!(var_expr.identifier.data, "_private");
    }

    #[test]
    fn test_parse_alphanumeric_identifier() {
        let rule = VarRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("var123".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);

        let result = rule.parse(&mut parser);

        assert!(result.is_some());
        let var_expr = result.unwrap();
        assert_eq!(var_expr.identifier.data, "var123");
    }
}

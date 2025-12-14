use std::fmt;

use crate::ast::declaration_expr::DeclarationExpr;
use crate::diagnostic::ErrMsg;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::{expr::ExprRule, parsed_type::ParsedTypeRule};
use crate::lexer::token::TokenType;

pub struct DeclarationRule {}

impl fmt::Display for DeclarationRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Declaration")
    }
}

impl ParseRule<DeclarationExpr> for DeclarationRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::Let).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<DeclarationExpr> {
        parser.begin_range();
        parser.try_consume(TokenType::Let)?;

        let declaration_type = parser.apply_rule(ParsedTypeRule {}, "declaration type", Some(ErrMsg::ExpectedType))?;
        let declaration_name = parser.consume_or_diagnostic(TokenType::AnyIdentifier)?.unwrap_identifier();

        parser.consume_or_diagnostic(TokenType::Assignment)?;

        let expr = parser.apply_rule(ExprRule {}, "declaration expression", Some(ErrMsg::ExpectedExpression))?;

        parser.consume_or_diagnostic(TokenType::Semicolon);

        Some(DeclarationExpr::new(declaration_name, declaration_type, expr, parser.end_range()))
    }    
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::{Token, TokenType, PositionRange};

    fn create_token(token_type: TokenType) -> Token {
        Token::new(token_type, PositionRange::zero())
    }

    #[test]
    fn test_declaration_rule_check_match_with_let() {
        let rule = DeclarationRule {};
        let tokens = vec![
            create_token(TokenType::Let),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_declaration_rule_check_match_without_let() {
        let rule = DeclarationRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(!rule.check_match(cursor));
    }

    #[test]
    fn test_parse_simple_declaration() {
        let rule = DeclarationRule {};
        let tokens = vec![
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let declaration = result.unwrap();
        assert_eq!(declaration.identifier.data, "x");
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid declaration");
    }

    #[test]
    fn test_parse_string_declaration() {
        let rule = DeclarationRule {};
        let tokens = vec![
            create_token(TokenType::Let),
            create_token(TokenType::Identifier("string".to_string())),
            create_token(TokenType::Identifier("message".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::StringLiteral("Hello World".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let declaration = result.unwrap();
        assert_eq!(declaration.identifier.data, "message");
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid string declaration");
    }

    #[test]
    fn test_parse_missing_let() {
        let rule = DeclarationRule {};
        let tokens = vec![
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because 'let' is required
        assert!(result.is_none());
        // No diagnostics expected here since the rule simply returns None when there's no 'let'
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_parse_missing_type() {
        let rule = DeclarationRule {};
        let tokens = vec![
            create_token(TokenType::Let),
            create_token(TokenType::Assignment), // Cannot be interpreted as a type
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because type is required
        assert!(result.is_none());
        // Should have diagnostic for expected type
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing type");
        assert!(diagnostics.iter().any(|d| d.message.contains("expected type")));
    }

    #[test]
    fn test_parse_missing_identifier() {
        let rule = DeclarationRule {};
        let tokens = vec![
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because identifier is required
        assert!(result.is_none());
        // Should have diagnostic for expected identifier
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing identifier");
        assert!(diagnostics.iter().any(|d| d.message.contains("identifier")));
    }

    #[test]
    fn test_parse_missing_assignment() {
        let rule = DeclarationRule {};
        let tokens = vec![
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because assignment operator is required
        assert!(result.is_none());
        // Should have diagnostic for expected assignment
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing assignment operator");
        assert!(diagnostics.iter().any(|d| d.message.contains("'='")));
    }

    #[test]
    fn test_parse_missing_expression() {
        let rule = DeclarationRule {};
        let tokens = vec![
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because expression is required
        assert!(result.is_none());
        // Should have diagnostic for expected expression
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing expression");
        assert!(diagnostics.iter().any(|d| d.message.contains("expected expression")));
    }

    #[test]
    fn test_parse_missing_semicolon() {
        let rule = DeclarationRule {};
        let tokens = vec![
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should still return Some but with diagnostics for missing semicolon
        assert!(result.is_some());
        let declaration = result.unwrap();
        assert_eq!(declaration.identifier.data, "x");
        // Should have diagnostic for missing semicolon
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing semicolon");
        assert!(diagnostics.iter().any(|d| d.message.contains("';'")));
    }

    #[test]
    fn test_parse_complex_expression() {
        let rule = DeclarationRule {};
        let tokens = vec![
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("result".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::Star),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let declaration = result.unwrap();
        assert_eq!(declaration.identifier.data, "result");
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid complex declaration");
    }

    #[test]
    fn test_parse_invalid_identifier() {
        let rule = DeclarationRule {};
        let tokens = vec![
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::IntLiteral(123)), // Invalid identifier
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because identifier must be AnyIdentifier
        assert!(result.is_none());
        // Should have diagnostic for expected identifier
        assert!(!diagnostics.is_empty(), "Expected diagnostic for invalid identifier");
        assert!(diagnostics.iter().any(|d| d.message.contains("identifier")));
    }

    #[test]
    fn test_parse_invalid_type() {
        let rule = DeclarationRule {};
        let tokens = vec![
            create_token(TokenType::Let),
            create_token(TokenType::IntLiteral(123)), // Invalid type
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because type is invalid
        assert!(result.is_none());
        // Should have diagnostic for expected type
        assert!(!diagnostics.is_empty(), "Expected diagnostic for invalid type");
        assert!(diagnostics.iter().any(|d| d.message.contains("expected type")));
    }

    #[test]
    fn test_parse_invalid_expression() {
        let rule = DeclarationRule {};
        let tokens = vec![
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::RightCurly), // Invalid expression
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because expression is invalid
        assert!(result.is_none());
        // Should have diagnostic for expected expression
        assert!(!diagnostics.is_empty(), "Expected diagnostic for invalid expression");
        assert!(diagnostics.iter().any(|d| d.message.contains("expected expression")));
    }

    #[test]
    fn test_parse_constructor_assignment() {
        let rule = DeclarationRule {};
        let tokens = vec![
            create_token(TokenType::Let),
            create_token(TokenType::Identifier("Person".to_string())),
            create_token(TokenType::Identifier("p".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::DollarSign),
            create_token(TokenType::Identifier("Person".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::StringLiteral("John".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let declaration = result.unwrap();
        assert_eq!(declaration.identifier.data, "p");
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid constructor assignment");
    }
}
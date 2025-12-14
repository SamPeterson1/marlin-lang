use std::fmt;

use crate::ast::delete_expr::DeleteExpr;
use crate::diagnostic::ErrMsg;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::expr::ExprRule;
use crate::lexer::token::TokenType;

pub struct DeleteRule {}

impl fmt::Display for DeleteRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Delete")
    }
}

impl ParseRule<DeleteExpr> for DeleteRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::Delete).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<DeleteExpr> {
        parser.begin_range();

        parser.try_consume(TokenType::Delete)?;
        let expr = parser.apply_rule(ExprRule {}, "delete expression", Some(ErrMsg::ExpectedExpression))?;
        parser.consume_or_diagnostic(TokenType::Semicolon);

        Some(DeleteExpr::new(expr, parser.end_range()))
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
    fn test_delete_rule_check_match_with_delete() {
        let rule = DeleteRule {};
        let tokens = vec![
            create_token(TokenType::Delete),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_delete_rule_check_match_without_delete() {
        let rule = DeleteRule {};
        let tokens = vec![
            create_token(TokenType::Let),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(!rule.check_match(cursor));
    }

    #[test]
    fn test_parse_delete_variable() {
        let rule = DeleteRule {};
        let tokens = vec![
            create_token(TokenType::Delete),
            create_token(TokenType::Identifier("variable".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for delete variable");
    }

    #[test]
    fn test_parse_delete_array_element() {
        let rule = DeleteRule {};
        let tokens = vec![
            create_token(TokenType::Delete),
            create_token(TokenType::Identifier("arr".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for delete array element");
    }

    #[test]
    fn test_parse_delete_nested_array_element() {
        let rule = DeleteRule {};
        let tokens = vec![
            create_token(TokenType::Delete),
            create_token(TokenType::Identifier("matrix".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for delete nested array element");
    }

    #[test]
    fn test_parse_delete_array_element_with_expression() {
        let rule = DeleteRule {};
        let tokens = vec![
            create_token(TokenType::Delete),
            create_token(TokenType::Identifier("arr".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::Identifier("index".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for delete array element with expression");
    }

    #[test]
    fn test_parse_delete_struct_member() {
        let rule = DeleteRule {};
        let tokens = vec![
            create_token(TokenType::Delete),
            create_token(TokenType::Identifier("obj".to_string())),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("field".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for delete struct member");
    }

    #[test]
    fn test_parse_delete_nested_member_access() {
        let rule = DeleteRule {};
        let tokens = vec![
            create_token(TokenType::Delete),
            create_token(TokenType::Identifier("obj".to_string())),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("nested".to_string())),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("field".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for delete nested member access");
    }

    #[test]
    fn test_parse_delete_array_in_struct() {
        let rule = DeleteRule {};
        let tokens = vec![
            create_token(TokenType::Delete),
            create_token(TokenType::Identifier("obj".to_string())),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("arr".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for delete array in struct");
    }

    #[test]
    fn test_parse_delete_struct_in_array() {
        let rule = DeleteRule {};
        let tokens = vec![
            create_token(TokenType::Delete),
            create_token(TokenType::Identifier("arr".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("field".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for delete struct in array");
    }

    #[test]
    fn test_parse_delete_complex_expression() {
        let rule = DeleteRule {};
        let tokens = vec![
            create_token(TokenType::Delete),
            create_token(TokenType::Identifier("objects".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::Identifier("getIndex".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::RightSquare),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("data".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::Identifier("key".to_string())),
            create_token(TokenType::RightSquare),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for delete complex expression");
    }

    #[test]
    fn test_parse_delete_with_function_call_index() {
        let rule = DeleteRule {};
        let tokens = vec![
            create_token(TokenType::Delete),
            create_token(TokenType::Identifier("arr".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::Identifier("findIndex".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Identifier("target".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::RightSquare),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for delete with function call index");
    }

    #[test]
    fn test_parse_delete_with_calculated_index() {
        let rule = DeleteRule {};
        let tokens = vec![
            create_token(TokenType::Delete),
            create_token(TokenType::Identifier("list".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::Identifier("start".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::Identifier("offset".to_string())),
            create_token(TokenType::Star),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for delete with calculated index");
    }

    #[test]
    fn test_parse_missing_expression() {
        let rule = DeleteRule {};
        let tokens = vec![
            create_token(TokenType::Delete),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because expression is required after delete
        assert!(result.is_none());
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing expression");
        assert!(diagnostics.iter().any(|d| d.message.contains("expected expression")));
    }

    #[test]
    fn test_parse_delete_parenthesized_expression() {
        let rule = DeleteRule {};
        let tokens = vec![
            create_token(TokenType::Delete),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Identifier("variable".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for delete parenthesized expression");
    }

    #[test]
    fn test_parse_delete_constructor_call_result() {
        let rule = DeleteRule {};
        let tokens = vec![
            create_token(TokenType::Delete),
            create_token(TokenType::DollarSign),
            create_token(TokenType::Identifier("Person".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::StringLiteral("John".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("name".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for delete constructor call result");
    }

    #[test]
    fn test_parse_delete_chained_member_access() {
        let rule = DeleteRule {};
        let tokens = vec![
            create_token(TokenType::Delete),
            create_token(TokenType::Identifier("world".to_string())),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("player".to_string())),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("inventory".to_string())),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("items".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for delete chained member access");
    }

    #[test]
    fn test_parse_delete_with_negative_index() {
        let rule = DeleteRule {};
        let tokens = vec![
            create_token(TokenType::Delete),
            create_token(TokenType::Identifier("arr".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::Minus),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for delete with negative index");
    }

    #[test]
    fn test_parse_delete_string_literal_index() {
        let rule = DeleteRule {};
        let tokens = vec![
            create_token(TokenType::Delete),
            create_token(TokenType::Identifier("dictionary".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::StringLiteral("key".to_string())),
            create_token(TokenType::RightSquare),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for delete string literal index");
    }

    #[test]
    fn test_parse_delete_missing_semicolon() {
        let rule = DeleteRule {};
        let tokens = vec![
            create_token(TokenType::Delete),
            create_token(TokenType::Identifier("variable".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should succeed but have diagnostic for missing semicolon
        assert!(result.is_some());
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing semicolon");
        assert!(diagnostics.iter().any(|d| d.message.contains("';'")));
    }

    #[test]
    fn test_parse_delete_invalid_expression() {
        let rule = DeleteRule {};
        let tokens = vec![
            create_token(TokenType::Delete),
            create_token(TokenType::RightCurly), // Invalid expression
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because expression is invalid
        assert!(result.is_none());
        assert!(!diagnostics.is_empty(), "Expected diagnostic for invalid expression");
        assert!(diagnostics.iter().any(|d| d.message.contains("expected expression")));
    }

    #[test]
    fn test_parse_delete_with_unary_expression() {
        let rule = DeleteRule {};
        let tokens = vec![
            create_token(TokenType::Delete),
            create_token(TokenType::Star),
            create_token(TokenType::Identifier("pointer".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for delete with unary expression");
    }
}
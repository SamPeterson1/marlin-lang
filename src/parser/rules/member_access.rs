use std::fmt;

use crate::ast::{ASTEnum, ArrayAccess, FunctionAccess, StructAccess};
use crate::diagnostic::ErrMsg;
use crate::parser::rules::arguments::ArgumentsRule;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::{expr::ExprRule, primary::PrimaryRule};
use crate::lexer::token::TokenType;

pub struct MemberAccessRule {}

impl fmt::Display for MemberAccessRule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "MemberAccess")
    }
}

impl ParseRule<ASTEnum> for MemberAccessRule {
    fn check_match(&self, _cursor: ParserCursor) -> bool {
        true
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<ASTEnum> {
        parser.begin_range();
        let mut expr = parser.apply_rule(PrimaryRule {}, "member access expression", Some(ErrMsg::ExpectedExpression))?;

        while let Some(token) = parser.try_match(&[TokenType::Dot, TokenType::Arrow, TokenType::LeftSquare, TokenType::LeftParen]) {
            parser.begin_range();
            
            if token.value == TokenType::Dot {
                parser.next();
                let identifier = parser.consume_or_diagnostic(TokenType::AnyIdentifier)?;
                let position = parser.end_range();
                
                expr = Box::new(StructAccess::new(expr, identifier.unwrap_identifier(), true, position)).into();
            } else if token.value == TokenType::Arrow {
                parser.next();
                let identifier = parser.consume_or_diagnostic(TokenType::AnyIdentifier)?;
                let position = parser.end_range();
                
                expr = Box::new(StructAccess::new(expr, identifier.unwrap_identifier(), false, position)).into();
            } else if token.value == TokenType::LeftSquare {
                parser.next();
                let index_expr = parser.apply_rule(ExprRule {}, "array index expression", Some(ErrMsg::ExpectedExpression))?;
                parser.consume_or_diagnostic(TokenType::RightSquare);
                let position = parser.end_range();
                
                expr = Box::new(ArrayAccess::new(expr, index_expr, position)).into();
            } else if token.value == TokenType::LeftParen {
                let arguments = parser.apply_rule(ArgumentsRule {}, "member access arguments", Some(ErrMsg::ExpectedArguments))?;
                let position = parser.end_range();
                
                expr = Box::new(FunctionAccess::new(expr, arguments, position)).into();
            }
        }

        Some(expr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::{Token, TokenType, PositionRange};
    use crate::logger::CONSOLE_LOGGER;

    fn create_token(token_type: TokenType) -> Token {
        Token::new(token_type, PositionRange::zero())
    }

    #[test]
    fn test_member_access_rule_check_match_always_true() {
        let rule = MemberAccessRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("obj".to_string())),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        // MemberAccessRule always returns true for check_match
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_parse_simple_variable_access() {
        let rule = MemberAccessRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("variable".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for simple variable access");
    }

    #[test]
    fn test_parse_direct_member_access() {
        let rule = MemberAccessRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("obj".to_string())),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("field".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for direct member access");
    }

    #[test]
    fn test_parse_indirect_member_access() {
        let rule = MemberAccessRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("ptr".to_string())),
            create_token(TokenType::Arrow),
            create_token(TokenType::Identifier("value".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for indirect member access");
    }

    #[test]
    fn test_parse_array_access() {
        let rule = MemberAccessRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("arr".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for array access");
    }

    #[test]
    fn test_parse_chained_member_access() {
        let rule = MemberAccessRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("obj".to_string())),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("nested".to_string())),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("field".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for chained member access");
    }

    #[test]
    fn test_parse_mixed_access_types() {
        let rule = MemberAccessRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("obj".to_string())),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("array".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::Arrow),
            create_token(TokenType::Identifier("value".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for mixed access types");
    }

    #[test]
    fn test_parse_nested_array_access() {
        let rule = MemberAccessRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("matrix".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for nested array access");
    }

    #[test]
    fn test_parse_array_access_with_variable_index() {
        let rule = MemberAccessRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("arr".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::Identifier("index".to_string())),
            create_token(TokenType::RightSquare),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for array access with variable index");
    }

    #[test]
    fn test_parse_array_access_with_expression_index() {
        let rule = MemberAccessRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("arr".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for array access with expression index");
    }

    #[test]
    fn test_parse_complex_chaining() {
        let rule = MemberAccessRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("world".to_string())),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("players".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("inventory".to_string())),
            create_token(TokenType::Arrow),
            create_token(TokenType::Identifier("damage".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for complex chaining");
    }

    #[test]
    fn test_parse_missing_member_after_dot() {
        let rule = MemberAccessRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("obj".to_string())),
            create_token(TokenType::Dot),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because identifier is required after dot
        assert!(result.is_none());
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing member after dot");
    }

    #[test]
    fn test_parse_missing_member_after_arrow() {
        let rule = MemberAccessRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("ptr".to_string())),
            create_token(TokenType::Arrow),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because identifier is required after arrow
        assert!(result.is_none());
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing member after arrow");
    }

    #[test]
    fn test_parse_missing_array_index() {
        let rule = MemberAccessRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("arr".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::RightSquare),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because expression is required for array index
        assert!(result.is_none());
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing array index");
        assert!(diagnostics.iter().any(|d| d.message.contains("expected expression")));
    }

    #[test]
    fn test_parse_missing_closing_bracket() {
        let rule = MemberAccessRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("arr".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should succeed but have diagnostic for missing bracket
        assert!(result.is_some());
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing closing bracket");
        assert!(diagnostics.iter().any(|d| d.message.contains("']'")));
    }

    #[test]
    fn test_parse_invalid_array_index() {
        let rule = MemberAccessRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("arr".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::RightCurly), // Invalid expression
            create_token(TokenType::RightSquare),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because array index expression is invalid
        assert!(result.is_none());
        assert!(!diagnostics.is_empty(), "Expected diagnostic for invalid array index");
        assert!(diagnostics.iter().any(|d| d.message.contains("expected expression")));
    }

    #[test]
    fn test_parse_literal_with_member_access() {
        let rule = MemberAccessRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("field".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for literal with member access");
    }

    #[test]
    fn test_parse_parenthesized_expression_with_access() {
        let rule = MemberAccessRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::Identifier("y".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("field".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for parenthesized expression with access");
    }

    #[test]
    fn test_parse_string_literal_with_array_access() {
        let rule = MemberAccessRule {};
        let tokens = vec![
            create_token(TokenType::StringLiteral("hello".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for string literal with array access");
    }
}
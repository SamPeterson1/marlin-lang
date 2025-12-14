use std::fmt;

use crate::ast::{ASTNode, member_access::{AccessType, MemberAccess}};
use crate::diagnostic::ErrMsg;
use crate::logger::Log;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::{arguments::ArgumentsRule, expr::ExprRule, primary::PrimaryRule};
use crate::lexer::token::TokenType;

pub struct MemberAccessRule {}

impl fmt::Display for MemberAccessRule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "MemberAccess")
    }
}

impl ParseRule<Box<dyn ASTNode>> for MemberAccessRule {
    fn check_match(&self, _cursor: ParserCursor) -> bool {
        true
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        parser.begin_range();
        let expr = parser.apply_rule(PrimaryRule {}, "member access expression", Some(ErrMsg::ExpectedExpression))?;

        let mut member_accesses = Vec::new();

        parser.log_debug(&format!("Parsing member access for expression at token {:?}", parser.cur()));

        while let Some(token) = parser.try_match(&[TokenType::Dot, TokenType::Arrow, TokenType::LeftSquare, TokenType::LeftParen]) {
            
            if token.value == TokenType::Dot {
                parser.next();
                let identifier = parser.consume_or_diagnostic(TokenType::AnyIdentifier)?;

                member_accesses.push(AccessType::Direct(identifier.unwrap_identifier()));
            } else if token.value == TokenType::Arrow {
                parser.next();

                let identifier = parser.consume_or_diagnostic(TokenType::AnyIdentifier)?;

                member_accesses.push(AccessType::Indirect(identifier.unwrap_identifier()));
            } else if token.value == TokenType::LeftSquare {
                parser.next();

                let index_expr = parser.apply_rule(ExprRule {}, "array index expression", Some(ErrMsg::ExpectedExpression))?;

                parser.consume_or_diagnostic(TokenType::RightSquare);

                member_accesses.push(AccessType::Array(index_expr));
            } else if token.value == TokenType::LeftParen {
                let args = parser.apply_rule(ArgumentsRule {}, "function call arguments", Some(ErrMsg::ExpectedArguments))?;

                member_accesses.push(AccessType::FunctionCall(args));
            }
        }
        
        Some(Box::new(MemberAccess::new(expr, member_accesses, parser.end_range())))
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
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
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
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
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
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
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
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for array access");
    }

    #[test]
    fn test_parse_function_call() {
        let rule = MemberAccessRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("func".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::Comma),
            create_token(TokenType::StringLiteral("test".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for function call");
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
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
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
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for mixed access types");
    }

    #[test]
    fn test_parse_method_call_chaining() {
        let rule = MemberAccessRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("obj".to_string())),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("method1".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("method2".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for method call chaining");
    }

    #[test]
    fn test_parse_array_of_objects() {
        let rule = MemberAccessRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("objects".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("getName".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for array of objects access");
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
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
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
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
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
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for array access with expression index");
    }

    #[test]
    fn test_parse_function_call_with_member_access_args() {
        let rule = MemberAccessRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("func".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Identifier("obj".to_string())),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("value".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for function call with member access args");
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
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("getItem".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::StringLiteral("sword".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::Arrow),
            create_token(TokenType::Identifier("damage".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
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
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
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
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
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
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
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
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
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
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because array index expression is invalid
        assert!(result.is_none());
        assert!(!diagnostics.is_empty(), "Expected diagnostic for invalid array index");
        assert!(diagnostics.iter().any(|d| d.message.contains("expected expression")));
    }

    #[test]
    fn test_parse_malformed_function_arguments() {
        let rule = MemberAccessRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("func".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::Comma),
            // Missing second argument after comma
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because function arguments parsing fails
        assert!(result.is_none());
        assert!(!diagnostics.is_empty(), "Expected diagnostic for malformed function arguments");
        assert!(diagnostics.iter().any(|d| d.message.contains("expected expression")));
    }

    #[test]
    fn test_parse_literal_with_member_access() {
        let rule = MemberAccessRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("toString".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for literal with member access");
    }

    #[test]
    fn test_parse_parenthesized_expression_with_access() {
        let rule = MemberAccessRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::Identifier("getValue".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("field".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for parenthesized expression with access");
    }

    #[test]
    fn test_parse_constructor_call_with_member_access() {
        let rule = MemberAccessRule {};
        let tokens = vec![
            create_token(TokenType::DollarSign),
            create_token(TokenType::Identifier("Person".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::StringLiteral("John".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("getName".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for constructor call with member access");
    }

    #[test]
    fn test_parse_array_access_with_function_call_index() {
        let rule = MemberAccessRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("arr".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::Identifier("getIndex".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::RightSquare),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for array access with function call index");
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
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for string literal with array access");
    }
}
use std::fmt;

use crate::ast::{ASTNode, FunctionCall, FunctionItem};
use crate::diagnostic::ErrMsg;
use crate::parser::rules::arguments::ArgumentsRule;
use crate::parser::rules::member_access::MemberAccessRule;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::{block::BlockRule, parameters::ParametersRule, parsed_type::ParsedTypeRule};
use crate::lexer::token::TokenType;

pub struct FunctionCallRule;

impl fmt::Display for FunctionCallRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FunctionCall")
    }
}

impl ParseRule<Box<dyn ASTNode>> for FunctionCallRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        true
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        parser.begin_range();

        let member_access = parser.apply_rule(MemberAccessRule {}, "member access", None)?;
        
        if let Some(arguments) = parser.apply_rule(ArgumentsRule {}, "function call arguments", None) {
            Some(Box::new(FunctionCall::new(member_access, arguments, parser.end_range())))
        } else {
            Some(member_access)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::{Token, TokenType, PositionRange};
    use crate::parser::ExprParser;

    fn create_token(token_type: TokenType) -> Token {
        Token::new(token_type, PositionRange::zero())
    }

    #[test]
    fn test_simple_function_call() {
        let rule = FunctionCallRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("foo".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for simple function call");
    }

    #[test]
    fn test_function_call_with_single_argument() {
        let rule = FunctionCallRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("print".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for function call with single argument");
    }

    #[test]
    fn test_function_call_with_multiple_arguments() {
        let rule = FunctionCallRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("add".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Comma),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for function call with multiple arguments");
    }

    #[test]
    fn test_function_call_with_variable_arguments() {
        let rule = FunctionCallRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("calculate".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Comma),
            create_token(TokenType::Identifier("y".to_string())),
            create_token(TokenType::Comma),
            create_token(TokenType::Identifier("z".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for function call with variable arguments");
    }

    #[test]
    fn test_function_call_with_expression_arguments() {
        let rule = FunctionCallRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("max".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Identifier("a".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Comma),
            create_token(TokenType::Identifier("b".to_string())),
            create_token(TokenType::Star),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for function call with expression arguments");
    }

    #[test]
    fn test_nested_function_calls() {
        let rule = FunctionCallRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("outer".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Identifier("inner".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::RightParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for nested function calls");
    }

    #[test]
    fn test_method_call() {
        let rule = FunctionCallRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("obj".to_string())),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("method".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for method call");
    }

    #[test]
    fn test_method_call_with_arguments() {
        let rule = FunctionCallRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("obj".to_string())),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("setValue".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for method call with arguments");
    }

    #[test]
    fn test_chained_method_calls() {
        let rule = FunctionCallRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("obj".to_string())),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("getData".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("process".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for chained method calls");
    }

    #[test]
    fn test_pointer_method_call() {
        let rule = FunctionCallRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("ptr".to_string())),
            create_token(TokenType::Arrow),
            create_token(TokenType::Identifier("method".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for pointer method call");
    }

    #[test]
    fn test_array_element_method_call() {
        let rule = FunctionCallRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("arr".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("process".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for array element method call");
    }

    #[test]
    fn test_function_call_with_string_literal() {
        let rule = FunctionCallRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("print".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::StringLiteral("Hello, World!".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for function call with string literal");
    }

    #[test]
    fn test_function_call_with_boolean_arguments() {
        let rule = FunctionCallRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("toggle".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::Comma),
            create_token(TokenType::BoolLiteral(false)),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for function call with boolean arguments");
    }

    #[test]
    fn test_function_call_with_char_literal() {
        let rule = FunctionCallRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("printChar".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::CharLiteral('A')),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for function call with char literal");
    }

    #[test]
    fn test_just_member_access_without_call() {
        let rule = FunctionCallRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("variable".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should return the member access without wrapping in FunctionCall
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for member access without call");
    }

    #[test]
    fn test_member_access_without_arguments() {
        let rule = FunctionCallRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("obj".to_string())),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("field".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should return the member access without wrapping in FunctionCall
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for member access without arguments");
    }

    #[test]
    fn test_function_call_missing_closing_paren() {
        let rule = FunctionCallRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("foo".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should still succeed but have diagnostic about missing paren
        assert!(result.is_some());
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing closing paren");
        assert!(diagnostics.iter().any(|d| d.message.contains("')'") || d.message.contains("expected")));
    }

    #[test]
    fn test_function_call_trailing_comma() {
        let rule = FunctionCallRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("foo".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Comma),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Behavior depends on ArgumentsRule - might fail or succeed with diagnostic
        assert!(result.is_some());
    }

    #[test]
    fn test_complex_member_access_with_call() {
        let rule = FunctionCallRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("world".to_string())),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("entities".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::Arrow),
            create_token(TokenType::Identifier("update".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::DoubleLiteral(0.016)),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for complex member access with call");
    }

    #[test]
    fn test_function_call_with_mixed_argument_types() {
        let rule = FunctionCallRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("process".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::Comma),
            create_token(TokenType::StringLiteral("test".to_string())),
            create_token(TokenType::Comma),
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::Comma),
            create_token(TokenType::DoubleLiteral(3.14)),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for function call with mixed argument types");
    }

    #[test]
    fn test_function_call_as_argument() {
        let rule = FunctionCallRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("outer".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Identifier("inner".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::RightParen),
            create_token(TokenType::Comma),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for function call as argument");
    }

    #[test]
    fn test_parenthesized_expression_with_call() {
        let rule = FunctionCallRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::Identifier("getFunc".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for parenthesized expression with call");
    }
}
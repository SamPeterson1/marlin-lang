use std::fmt;

use crate::ast::ImplItem;
use crate::diagnostic::ErrMsg;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::{function_item::FunctionRule, parsed_type::ParsedTypeRule};
use crate::lexer::token::TokenType;

pub struct ImplBlockRule {}

impl fmt::Display for ImplBlockRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ImplBlock")
    }
}

impl ParseRule<ImplItem> for ImplBlockRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::Impl).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<ImplItem> {
        parser.begin_range();
        parser.try_consume(TokenType::Impl)?;

        let impl_type = parser.apply_rule(ParsedTypeRule {}, "impl type", Some(ErrMsg::ExpectedType))?;

        parser.consume_or_diagnostic(TokenType::LeftCurly);

        let mut functions = Vec::new();

        while let Some(function) = parser.apply_rule(FunctionRule { }, "impl function", None) {
            functions.push(function);
        }

        parser.consume_or_diagnostic(TokenType::RightCurly);

        Some(ImplItem::new(impl_type, functions, parser.end_range()))
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
    fn test_impl_block_check_match_with_impl_keyword() {
        let rule = ImplBlockRule {};
        let tokens = vec![
            create_token(TokenType::Impl),
            create_token(TokenType::Identifier("MyType".to_string())),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_impl_block_check_match_without_impl_keyword() {
        let rule = ImplBlockRule {};
        let tokens = vec![
            create_token(TokenType::Struct),
            create_token(TokenType::Identifier("MyType".to_string())),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(!rule.check_match(cursor));
    }

    #[test]
    fn test_parse_empty_impl_block() {
        let rule = ImplBlockRule {};
        let tokens = vec![
            create_token(TokenType::Impl),
            create_token(TokenType::Identifier("MyType".to_string())),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let impl_item = result.unwrap();
        assert_eq!(impl_item.functions.len(), 0);
    }

    #[test]
    fn test_parse_impl_block_with_single_function() {
        let rule = ImplBlockRule {};
        let tokens = vec![
            create_token(TokenType::Impl),
            create_token(TokenType::Identifier("MyStruct".to_string())),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Fn),
            create_token(TokenType::Identifier("get_value".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::Arrow),
            create_token(TokenType::Int),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Return),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let impl_item = result.unwrap();
        assert_eq!(impl_item.functions.len(), 1);
        assert_eq!(impl_item.functions[0].name.data, "get_value");
    }

    #[test]
    fn test_parse_impl_block_with_multiple_functions() {
        let rule = ImplBlockRule {};
        let tokens = vec![
            create_token(TokenType::Impl),
            create_token(TokenType::Identifier("Calculator".to_string())),
            create_token(TokenType::LeftCurly),
            // First function: add
            create_token(TokenType::Fn),
            create_token(TokenType::Identifier("add".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("a".to_string())),
            create_token(TokenType::Comma),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("b".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::Arrow),
            create_token(TokenType::Int),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Return),
            create_token(TokenType::Identifier("a".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::Identifier("b".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            // Second function: multiply
            create_token(TokenType::Fn),
            create_token(TokenType::Identifier("multiply".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Comma),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("y".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::Arrow),
            create_token(TokenType::Int),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Return),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Star),
            create_token(TokenType::Identifier("y".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let impl_item = result.unwrap();
        assert_eq!(impl_item.functions.len(), 2);
        assert_eq!(impl_item.functions[0].name.data, "add");
        assert_eq!(impl_item.functions[1].name.data, "multiply");
    }

    #[test]
    fn test_parse_impl_block_missing_type() {
        let rule = ImplBlockRule {};
        let tokens = vec![
            create_token(TokenType::Impl),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        let result = rule.parse(&mut parser);
        
        // Should fail to parse due to missing type
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_impl_block_missing_opening_brace() {
        let rule = ImplBlockRule {};
        let tokens = vec![
            create_token(TokenType::Impl),
            create_token(TokenType::Identifier("MyType".to_string())),
            create_token(TokenType::Fn),
            create_token(TokenType::Identifier("test".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::Arrow),
            create_token(TokenType::Int),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Return),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        let result = rule.parse(&mut parser);
        
        // Should still parse the impl part but generate diagnostic for missing brace
        assert!(result.is_some());
    }

    #[test]
    fn test_parse_impl_block_missing_closing_brace() {
        let rule = ImplBlockRule {};
        let tokens = vec![
            create_token(TokenType::Impl),
            create_token(TokenType::Identifier("MyType".to_string())),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Fn),
            create_token(TokenType::Identifier("test".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::Arrow),
            create_token(TokenType::Int),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Return),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        let result = rule.parse(&mut parser);
        
        // Should parse but generate diagnostic for missing closing brace
        assert!(result.is_some());
    }

    #[test]
    fn test_parse_impl_block_with_invalid_function() {
        let rule = ImplBlockRule {};
        let tokens = vec![
            create_token(TokenType::Impl),
            create_token(TokenType::Identifier("MyType".to_string())),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Fn),
            create_token(TokenType::Identifier("incomplete_function".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        let result = rule.parse(&mut parser);
        
        // Should parse the impl block structure but have errors from the malformed function
        assert!(result.is_some());
        let impl_item = result.unwrap();
        // The malformed function shouldn't be added to the functions list
        assert_eq!(impl_item.functions.len(), 0);
    }

    #[test]
    fn test_parse_impl_block_with_generic_type() {
        let rule = ImplBlockRule {};
        let tokens = vec![
            create_token(TokenType::Impl),
            create_token(TokenType::Identifier("Array".to_string())),
            create_token(TokenType::Less),
            create_token(TokenType::Int),
            create_token(TokenType::Greater),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        let result = rule.parse(&mut parser);
        
        // May succeed or fail depending on how generic types are handled
        // This test documents the current behavior
        assert!(result.is_some());
        let impl_item = result.unwrap();
        assert_eq!(impl_item.functions.len(), 0);
    }

    #[test]
    fn test_parse_impl_block_empty_input() {
        let rule = ImplBlockRule {};
        let tokens = vec![create_token(TokenType::EOF)];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        let result = rule.parse(&mut parser);
        
        assert!(result.is_none());
    }

    #[test]
    fn test_impl_block_check_match_empty_input() {
        let rule = ImplBlockRule {};
        let tokens = vec![create_token(TokenType::EOF)];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(!rule.check_match(cursor));
    }

    #[test]
    fn test_display_trait() {
        let rule = ImplBlockRule {};
        assert_eq!(format!("{}", rule), "ImplBlock");
    }
}
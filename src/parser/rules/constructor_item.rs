use std::fmt;

use crate::ast::constructor_item::ConstructorItem;
use crate::diagnostic::ErrMsg;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::{block::BlockRule, parameters::ParametersRule};
use crate::lexer::token::TokenType;

pub struct ConstructorRule {}

impl fmt::Display for ConstructorRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Constructor")
    }
}

impl ParseRule<ConstructorItem> for ConstructorRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::DollarSign).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<ConstructorItem> {
        parser.begin_range();
        parser.try_consume(TokenType::DollarSign)?;

        let parameters = parser.apply_rule(ParametersRule {}, "constructor parameters", Some(ErrMsg::ExpectedParameters))?;
        let body = parser.apply_rule(BlockRule {}, "constructor body", Some(ErrMsg::ExpectedBlock))?;        
        
        Some(ConstructorItem::new(parameters, body, parser.end_range()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::{Token, TokenType, PositionRange};
    use crate::parser::ExprParser;

    fn create_parser_with_tokens(tokens: Vec<TokenType>) -> ExprParser<'static> {
        let diagnostics = Box::leak(Box::new(Vec::new()));
        let tokens: Vec<Token> = tokens
            .into_iter()
            .map(|token_type| Token::new(token_type, PositionRange::zero()))
            .collect();
        ExprParser::new(tokens, diagnostics)
    }

    #[test]
    fn test_constructor_check_match_with_dollar_sign() {
        let parser = create_parser_with_tokens(vec![
            TokenType::DollarSign,
            TokenType::LeftParen,
            TokenType::RightParen,
            TokenType::LeftCurly,
            TokenType::RightCurly,
            TokenType::EOF,
        ]);
        let rule = ConstructorRule {};
        assert!(rule.check_match(parser.get_cursor()));
    }

    #[test]
    fn test_constructor_check_match_without_dollar_sign() {
        let parser = create_parser_with_tokens(vec![
            TokenType::Fn,
            TokenType::Identifier("test".to_string()),
            TokenType::LeftParen,
            TokenType::RightParen,
            TokenType::LeftCurly,
            TokenType::RightCurly,
            TokenType::EOF,
        ]);
        let rule = ConstructorRule {};
        assert!(!rule.check_match(parser.get_cursor()));
    }

    #[test]
    fn test_parse_simple_constructor() {
        let mut parser = create_parser_with_tokens(vec![
            TokenType::DollarSign,
            TokenType::LeftParen,
            TokenType::RightParen,
            TokenType::LeftCurly,
            TokenType::RightCurly,
            TokenType::EOF,
        ]);
        let rule = ConstructorRule {};
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let constructor = result.unwrap();
        assert_eq!(constructor.parameters.parameters.len(), 0);
    }

    #[test]
    fn test_parse_constructor_with_parameters() {
        let mut parser = create_parser_with_tokens(vec![
            TokenType::DollarSign,
            TokenType::LeftParen,
            TokenType::Int,
            TokenType::Identifier("x".to_string()),
            TokenType::Comma,
            TokenType::Int,
            TokenType::Identifier("y".to_string()),
            TokenType::RightParen,
            TokenType::LeftCurly,
            TokenType::RightCurly,
            TokenType::EOF,
        ]);
        let rule = ConstructorRule {};
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let constructor = result.unwrap();
        assert_eq!(constructor.parameters.parameters.len(), 2);
    }

    #[test]
    fn test_parse_constructor_with_body() {
        let mut parser = create_parser_with_tokens(vec![
            TokenType::DollarSign,
            TokenType::LeftParen,
            TokenType::Int,
            TokenType::Identifier("value".to_string()),
            TokenType::RightParen,
            TokenType::LeftCurly,
            TokenType::Identifier("this".to_string()),
            TokenType::Dot,
            TokenType::Identifier("value".to_string()),
            TokenType::Assignment,
            TokenType::Identifier("value".to_string()),
            TokenType::Semicolon,
            TokenType::RightCurly,
            TokenType::EOF,
        ]);
        let rule = ConstructorRule {};
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let constructor = result.unwrap();
        assert_eq!(constructor.parameters.parameters.len(), 1);
        // Body should contain the assignment statement
        assert_eq!(constructor.body.exprs.len(), 1);
    }

    #[test]
    fn test_parse_constructor_with_multiple_parameters() {
        let mut parser = create_parser_with_tokens(vec![
            TokenType::DollarSign,
            TokenType::LeftParen,
            TokenType::Int,
            TokenType::Identifier("width".to_string()),
            TokenType::Comma,
            TokenType::Int,
            TokenType::Identifier("height".to_string()),
            TokenType::Comma,
            TokenType::Bool,
            TokenType::Identifier("visible".to_string()),
            TokenType::RightParen,
            TokenType::LeftCurly,
            TokenType::Identifier("this".to_string()),
            TokenType::Dot,
            TokenType::Identifier("width".to_string()),
            TokenType::Assignment,
            TokenType::Identifier("width".to_string()),
            TokenType::Semicolon,
            TokenType::Identifier("this".to_string()),
            TokenType::Dot,
            TokenType::Identifier("height".to_string()),
            TokenType::Assignment,
            TokenType::Identifier("height".to_string()),
            TokenType::Semicolon,
            TokenType::Identifier("this".to_string()),
            TokenType::Dot,
            TokenType::Identifier("visible".to_string()),
            TokenType::Assignment,
            TokenType::Identifier("visible".to_string()),
            TokenType::Semicolon,
            TokenType::RightCurly,
            TokenType::EOF,
        ]);
        let rule = ConstructorRule {};
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let constructor = result.unwrap();
        assert_eq!(constructor.parameters.parameters.len(), 3);
        assert_eq!(constructor.body.exprs.len(), 3);
    }

    #[test]
    fn test_parse_constructor_missing_parameters() {
        let mut parser = create_parser_with_tokens(vec![
            TokenType::DollarSign,
            TokenType::LeftCurly,
            TokenType::RightCurly,
            TokenType::EOF,
        ]);
        let rule = ConstructorRule {};
        let result = rule.parse(&mut parser);
        
        // Should fail to parse due to missing parameters
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_constructor_missing_body() {
        let mut parser = create_parser_with_tokens(vec![
            TokenType::DollarSign,
            TokenType::LeftParen,
            TokenType::RightParen,
            TokenType::EOF,
        ]);
        let rule = ConstructorRule {};
        let result = rule.parse(&mut parser);
        
        // Should fail to parse due to missing body
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_constructor_malformed_parameters() {
        let mut parser = create_parser_with_tokens(vec![
            TokenType::DollarSign,
            TokenType::LeftParen,
            TokenType::Int,
            TokenType::Comma,
            TokenType::Identifier("x".to_string()),
            TokenType::RightParen,
            TokenType::LeftCurly,
            TokenType::RightCurly,
            TokenType::EOF,
        ]);
        let rule = ConstructorRule {};
        let result = rule.parse(&mut parser);
        
        // Should fail due to malformed parameters (missing parameter name)
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_constructor_with_nested_blocks() {
        let mut parser = create_parser_with_tokens(vec![
            TokenType::DollarSign,
            TokenType::LeftParen,
            TokenType::Int,
            TokenType::Identifier("value".to_string()),
            TokenType::RightParen,
            TokenType::LeftCurly,
            TokenType::If,
            TokenType::Identifier("value".to_string()),
            TokenType::Greater,
            TokenType::IntLiteral(0),
            TokenType::LeftCurly,
            TokenType::Identifier("this".to_string()),
            TokenType::Dot,
            TokenType::Identifier("value".to_string()),
            TokenType::Equal,
            TokenType::Identifier("value".to_string()),
            TokenType::Semicolon,
            TokenType::RightCurly,
            TokenType::Else,
            TokenType::LeftCurly,
            TokenType::Identifier("this".to_string()),
            TokenType::Dot,
            TokenType::Identifier("value".to_string()),
            TokenType::Equal,
            TokenType::IntLiteral(0),
            TokenType::Semicolon,
            TokenType::RightCurly,
            TokenType::RightCurly,
            TokenType::EOF,
        ]);
        let rule = ConstructorRule {};
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let constructor = result.unwrap();
        assert_eq!(constructor.parameters.parameters.len(), 1);
        // Should contain the if statement
        assert_eq!(constructor.body.exprs.len(), 1);
    }

    #[test]
    fn test_constructor_check_match_empty_input() {
        let parser = create_parser_with_tokens(vec![TokenType::EOF]);
        let rule = ConstructorRule {};
        
        assert!(!rule.check_match(parser.get_cursor()));
    }

    #[test]
    fn test_parse_constructor_empty_input() {
        let mut parser = create_parser_with_tokens(vec![TokenType::EOF]);
        let rule = ConstructorRule {};
        let result = rule.parse(&mut parser);
        
        assert!(result.is_none());
    }

    #[test]
    fn test_display_trait() {
        let rule = ConstructorRule {};
        assert_eq!(format!("{}", rule), "Constructor");
    }
}
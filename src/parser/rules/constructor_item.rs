use std::fmt;

use crate::ast::ConstructorItem;
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

    #[test]
    fn test_constructor_check_match_with_dollar_sign() {
        let tokens = vec![
            Token::new(TokenType::DollarSign, PositionRange::zero()),
            Token::new(TokenType::LeftParen, PositionRange::zero()),
            Token::new(TokenType::RightParen, PositionRange::zero()),
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::EOF, PositionRange::zero()),
        ];
        let mut diagnostics = Vec::new();
        let parser = ExprParser::new(tokens, &mut diagnostics);
        let rule = ConstructorRule {};
        assert!(rule.check_match(parser.get_cursor()));
    }

    #[test]
    fn test_constructor_check_match_without_dollar_sign() {
        let tokens = vec![
            Token::new(TokenType::Fn, PositionRange::zero()),
            Token::new(TokenType::Identifier("test".to_string()), PositionRange::zero()),
            Token::new(TokenType::LeftParen, PositionRange::zero()),
            Token::new(TokenType::RightParen, PositionRange::zero()),
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::EOF, PositionRange::zero()),
        ];
        let mut diagnostics = Vec::new();
        let parser = ExprParser::new(tokens, &mut diagnostics);
        let rule = ConstructorRule {};
        assert!(!rule.check_match(parser.get_cursor()));
    }

    #[test]
    fn test_parse_simple_constructor() {
        let tokens = vec![
            Token::new(TokenType::DollarSign, PositionRange::zero()),
            Token::new(TokenType::LeftParen, PositionRange::zero()),
            Token::new(TokenType::RightParen, PositionRange::zero()),
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::EOF, PositionRange::zero()),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        let rule = ConstructorRule {};
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let constructor = result.unwrap();
        assert_eq!(constructor.parameters.parameters.len(), 0);
    }

    #[test]
    fn test_parse_constructor_with_parameters() {
        let tokens = vec![
            Token::new(TokenType::DollarSign, PositionRange::zero()),
            Token::new(TokenType::LeftParen, PositionRange::zero()),
            Token::new(TokenType::Int, PositionRange::zero()),
            Token::new(TokenType::Identifier("x".to_string()), PositionRange::zero()),
            Token::new(TokenType::Comma, PositionRange::zero()),
            Token::new(TokenType::Int, PositionRange::zero()),
            Token::new(TokenType::Identifier("y".to_string()), PositionRange::zero()),
            Token::new(TokenType::RightParen, PositionRange::zero()),
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::EOF, PositionRange::zero()),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        let rule = ConstructorRule {};
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let constructor = result.unwrap();
        assert_eq!(constructor.parameters.parameters.len(), 2);
    }

    #[test]
    fn test_parse_constructor_with_body() {
        let tokens = vec![
            Token::new(TokenType::DollarSign, PositionRange::zero()),
            Token::new(TokenType::LeftParen, PositionRange::zero()),
            Token::new(TokenType::Int, PositionRange::zero()),
            Token::new(TokenType::Identifier("value".to_string()), PositionRange::zero()),
            Token::new(TokenType::RightParen, PositionRange::zero()),
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::Identifier("this".to_string()), PositionRange::zero()),
            Token::new(TokenType::Dot, PositionRange::zero()),
            Token::new(TokenType::Identifier("value".to_string()), PositionRange::zero()),
            Token::new(TokenType::Assignment, PositionRange::zero()),
            Token::new(TokenType::Identifier("value".to_string()), PositionRange::zero()),
            Token::new(TokenType::Semicolon, PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::EOF, PositionRange::zero()),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
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
        let tokens = vec![
            Token::new(TokenType::DollarSign, PositionRange::zero()),
            Token::new(TokenType::LeftParen, PositionRange::zero()),
            Token::new(TokenType::Int, PositionRange::zero()),
            Token::new(TokenType::Identifier("width".to_string()), PositionRange::zero()),
            Token::new(TokenType::Comma, PositionRange::zero()),
            Token::new(TokenType::Int, PositionRange::zero()),
            Token::new(TokenType::Identifier("height".to_string()), PositionRange::zero()),
            Token::new(TokenType::Comma, PositionRange::zero()),
            Token::new(TokenType::Bool, PositionRange::zero()),
            Token::new(TokenType::Identifier("visible".to_string()), PositionRange::zero()),
            Token::new(TokenType::RightParen, PositionRange::zero()),
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::Identifier("this".to_string()), PositionRange::zero()),
            Token::new(TokenType::Dot, PositionRange::zero()),
            Token::new(TokenType::Identifier("width".to_string()), PositionRange::zero()),
            Token::new(TokenType::Assignment, PositionRange::zero()),
            Token::new(TokenType::Identifier("width".to_string()), PositionRange::zero()),
            Token::new(TokenType::Semicolon, PositionRange::zero()),
            Token::new(TokenType::Identifier("this".to_string()), PositionRange::zero()),
            Token::new(TokenType::Dot, PositionRange::zero()),
            Token::new(TokenType::Identifier("height".to_string()), PositionRange::zero()),
            Token::new(TokenType::Assignment, PositionRange::zero()),
            Token::new(TokenType::Identifier("height".to_string()), PositionRange::zero()),
            Token::new(TokenType::Semicolon, PositionRange::zero()),
            Token::new(TokenType::Identifier("this".to_string()), PositionRange::zero()),
            Token::new(TokenType::Dot, PositionRange::zero()),
            Token::new(TokenType::Identifier("visible".to_string()), PositionRange::zero()),
            Token::new(TokenType::Assignment, PositionRange::zero()),
            Token::new(TokenType::Identifier("visible".to_string()), PositionRange::zero()),
            Token::new(TokenType::Semicolon, PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::EOF, PositionRange::zero()),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        let rule = ConstructorRule {};
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let constructor = result.unwrap();
        assert_eq!(constructor.parameters.parameters.len(), 3);
        assert_eq!(constructor.body.exprs.len(), 3);
    }

    #[test]
    fn test_parse_constructor_missing_parameters() {
        let tokens = vec![
            Token::new(TokenType::DollarSign, PositionRange::zero()),
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::EOF, PositionRange::zero()),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        let rule = ConstructorRule {};
        let result = rule.parse(&mut parser);
        
        // Should fail to parse due to missing parameters
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_constructor_missing_body() {
        let tokens = vec![
            Token::new(TokenType::DollarSign, PositionRange::zero()),
            Token::new(TokenType::LeftParen, PositionRange::zero()),
            Token::new(TokenType::RightParen, PositionRange::zero()),
            Token::new(TokenType::EOF, PositionRange::zero()),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        let rule = ConstructorRule {};
        let result = rule.parse(&mut parser);
        
        // Should fail to parse due to missing body
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_constructor_malformed_parameters() {
        let tokens = vec![
            Token::new(TokenType::DollarSign, PositionRange::zero()),
            Token::new(TokenType::LeftParen, PositionRange::zero()),
            Token::new(TokenType::Int, PositionRange::zero()),
            Token::new(TokenType::Comma, PositionRange::zero()),
            Token::new(TokenType::Identifier("x".to_string()), PositionRange::zero()),
            Token::new(TokenType::RightParen, PositionRange::zero()),
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::EOF, PositionRange::zero()),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        let rule = ConstructorRule {};
        let result = rule.parse(&mut parser);
        
        // Should fail due to malformed parameters (missing parameter name)
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_constructor_with_nested_blocks() {
        let tokens = vec![
            Token::new(TokenType::DollarSign, PositionRange::zero()),
            Token::new(TokenType::LeftParen, PositionRange::zero()),
            Token::new(TokenType::Int, PositionRange::zero()),
            Token::new(TokenType::Identifier("value".to_string()), PositionRange::zero()),
            Token::new(TokenType::RightParen, PositionRange::zero()),
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::If, PositionRange::zero()),
            Token::new(TokenType::Identifier("value".to_string()), PositionRange::zero()),
            Token::new(TokenType::Greater, PositionRange::zero()),
            Token::new(TokenType::IntLiteral(0), PositionRange::zero()),
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::Identifier("this".to_string()), PositionRange::zero()),
            Token::new(TokenType::Dot, PositionRange::zero()),
            Token::new(TokenType::Identifier("value".to_string()), PositionRange::zero()),
            Token::new(TokenType::Equal, PositionRange::zero()),
            Token::new(TokenType::Identifier("value".to_string()), PositionRange::zero()),
            Token::new(TokenType::Semicolon, PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::Else, PositionRange::zero()),
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::Identifier("this".to_string()), PositionRange::zero()),
            Token::new(TokenType::Dot, PositionRange::zero()),
            Token::new(TokenType::Identifier("value".to_string()), PositionRange::zero()),
            Token::new(TokenType::Equal, PositionRange::zero()),
            Token::new(TokenType::IntLiteral(0), PositionRange::zero()),
            Token::new(TokenType::Semicolon, PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::EOF, PositionRange::zero()),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
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
        let tokens = vec![Token::new(TokenType::EOF, PositionRange::zero())];
        let mut diagnostics = Vec::new();
        let parser = ExprParser::new(tokens, &mut diagnostics);
        let rule = ConstructorRule {};
        
        assert!(!rule.check_match(parser.get_cursor()));
    }

    #[test]
    fn test_parse_constructor_empty_input() {
        let tokens = vec![Token::new(TokenType::EOF, PositionRange::zero())];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
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